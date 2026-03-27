package nerdfont

import (
	"encoding/json"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"

	"github.com/indium114/spyglass/lens"
)

const glyphURL = "https://raw.githubusercontent.com/ryanoasis/nerd-fonts/refs/heads/master/glyphnames.json"

type nerdFontLens struct {
	mu     sync.RWMutex
	glyphs []glyphEntry
}

type glyphEntry struct {
	Name string
	Char string
	Code string
}

func New() lens.Lens {
	l := &nerdFontLens{}
	l.loadCache()
	return l
}

func (n *nerdFontLens) Name() string {
	return "NerdFont"
}

func cachePath() string {
	dir, _ := os.UserCacheDir()
	return filepath.Join(dir, "spyglass", "nerd-fonts", "glyphnames.json")
}

// Load cached glyphs
func (n *nerdFontLens) loadCache() {
	path := cachePath()
	data, err := os.ReadFile(path)
	if err != nil {
		go n.downloadGlyphs()
		return
	}

	var raw map[string]map[string]string
	if err := json.Unmarshal(data, &raw); err != nil {
		go n.downloadGlyphs()
		return
	}

	var glyphs []glyphEntry
	for name, entry := range raw {
		if c, ok := entry["char"]; ok {
			g := glyphEntry{
				Name: name,
				Char: c,
				Code: entry["code"],
			}
			glyphs = append(glyphs, g)
		}
	}

	n.mu.Lock()
	n.glyphs = glyphs
	n.mu.Unlock()
}

// Download glyph JSON and update cache
func (n *nerdFontLens) downloadGlyphs() {
	resp, err := http.Get(glyphURL)
	if err != nil {
		return
	}
	defer resp.Body.Close()

	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return
	}

	path := cachePath()
	_ = os.MkdirAll(filepath.Dir(path), 0755)
	_ = os.WriteFile(path, data, 0644)

	// Reload
	n.loadCache()
}

func (n *nerdFontLens) Search(query string) ([]lens.Entry, error) {
	n.mu.RLock()
	defer n.mu.RUnlock()

	q := strings.ToLower(strings.TrimSpace(query))
	var entries []lens.Entry
	for _, g := range n.glyphs {
		if q == "" || strings.Contains(strings.ToLower(g.Name), q) || strings.Contains(strings.ToLower(g.Char), q) {
			entries = append(entries, lens.Entry{
				ID:          g.Name,
				Title:       g.Name,
				Icon:        g.Char,
				Description: "Code: " + g.Code,
			})
		}
	}
	return entries, nil
}

func (n *nerdFontLens) Enter(e lens.Entry) error {
	// Copy to clipboard
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
	cmd.Start()
	in.Write([]byte(e.Icon))
	in.Close()
	cmd.Wait()

	return nil
}

func (n *nerdFontLens) ContextActions(e lens.Entry) []lens.Action {
	return []lens.Action{
		{
			Name: "Redownload glyph file",
			Run: func(entry lens.Entry) error {
				go n.downloadGlyphs()
				return nil
			},
		},
	}
}
