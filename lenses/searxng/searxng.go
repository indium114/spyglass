package searxng

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"sync"
	"syscall"
	"time"

	"github.com/indium114/spyglass/lens"
	"gopkg.in/yaml.v3"
)

type config struct {
	IP    string `yaml:"ip"`
	Port  int    `yaml:"port"`
	Limit int    `yaml:"limit"`
}

type result struct {
	Title string `json:"title"`
	URL   string `json:"url"`
}

type response struct {
	Results []result `json:"results"`
}

type searxLens struct {
	mu sync.RWMutex

	cfg config

	results   []result
	lastQuery string
	searching bool
}

func New() lens.Lens {
	l := &searxLens{}
	l.loadConfig()
	return l
}

func (l *searxLens) Name() string {
	return "SearXNG"
}

func configPath() string {
	home, _ := os.UserHomeDir()
	return filepath.Join(home, ".config", "spyglass", "searxng", "config.yaml")
}

func (l *searxLens) loadConfig() {
	data, err := os.ReadFile(configPath())
	if err != nil {
		return
	}

	var cfg config
	if err := yaml.Unmarshal(data, &cfg); err != nil {
		return
	}

	if cfg.Limit <= 0 {
		cfg.Limit = 25
	}

	l.mu.Lock()
	l.cfg = cfg
	l.mu.Unlock()
}

func (l *searxLens) Search(query string) ([]lens.Entry, error) {
	query = strings.TrimSpace(query)

	l.mu.Lock()

	// If empty query → clear results
	if query == "" {
		l.results = nil
		l.lastQuery = ""
		l.searching = false
		l.mu.Unlock()
		return nil, nil
	}

	// If new query and not already searching → fire async request
	if query != l.lastQuery && !l.searching {
		l.lastQuery = query
		l.searching = true
		go l.performSearch(query)
	}

	resultsCopy := make([]result, len(l.results))
	copy(resultsCopy, l.results)
	l.mu.Unlock()

	return l.buildEntries(resultsCopy), nil
}

func (l *searxLens) performSearch(query string) {
	l.mu.RLock()
	cfg := l.cfg
	l.mu.RUnlock()

	if cfg.IP == "" || cfg.Port == 0 {
		l.mu.Lock()
		l.searching = false
		l.mu.Unlock()
		return
	}

	endpoint := fmt.Sprintf(
		"http://%s:%d/search?q=%s&format=json",
		cfg.IP,
		cfg.Port,
		url.QueryEscape(query),
	)

	client := http.Client{Timeout: 5 * time.Second}
	resp, err := client.Get(endpoint)
	if err != nil {
		l.mu.Lock()
		l.searching = false
		l.mu.Unlock()
		return
	}
	defer resp.Body.Close()

	var parsed response
	if err := json.NewDecoder(resp.Body).Decode(&parsed); err != nil {
		l.mu.Lock()
		l.searching = false
		l.mu.Unlock()
		return
	}

	limit := cfg.Limit
	if limit <= 0 {
		limit = 25
	}
	if len(parsed.Results) > limit {
		parsed.Results = parsed.Results[:limit]
	}

	l.mu.Lock()
	l.results = parsed.Results
	l.searching = false
	l.mu.Unlock()
}

func (l *searxLens) buildEntries(results []result) []lens.Entry {
	var entries []lens.Entry

	for _, r := range results {
		domain := extractDomain(r.URL)

		entries = append(entries, lens.Entry{
			ID:          r.URL,
			Title:       fmt.Sprintf("(%s) %s", domain, r.Title),
			Icon:        "󰖟",
			Description: r.URL,
		})
	}

	return entries
}

func (l *searxLens) Enter(e lens.Entry) error {
	if runtime.GOOS == "darwin" {
		cmd := exec.Command("open", e.ID)
		cmd.Stdout = os.Stdout
		cmd.Stdin = os.Stdin
		cmd.Stderr = os.Stderr
		cmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

		return cmd.Start()
	}
	cmd := exec.Command("xdg-open", e.ID)
	cmd.Stdout = os.Stdout
	cmd.Stdin = os.Stdin
	cmd.Stderr = os.Stderr
	cmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

	return cmd.Start()
}

func (l *searxLens) ContextActions(e lens.Entry) []lens.Action {
	return []lens.Action{
		{
			Name: "Copy URL",
			Run: func(entry lens.Entry) error {
				return copyToClipboard(entry.ID)
			},
		},
	}
}

func extractDomain(raw string) string {
	u, err := url.Parse(raw)
	if err != nil {
		return ""
	}
	return u.Host
}

func copyToClipboard(text string) error {
	var cmd *exec.Cmd

	if _, err := exec.LookPath("wl-copy"); err == nil {
		cmd = exec.Command("wl-copy")
	} else if _, err := exec.LookPath("xclip"); err == nil {
		cmd = exec.Command("xclip", "-selection", "clipboard")
	} else if _, err := exec.LookPath("pbcopy"); err == nil {
		cmd = exec.Command("pbcopy")
	} else {
		return nil
	}

	in, _ := cmd.StdinPipe()
	if err := cmd.Start(); err != nil {
		return err
	}

	_, _ = in.Write([]byte(text))
	in.Close()
	return cmd.Wait()
}
