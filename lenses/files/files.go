package files

import (
	"encoding/json"
	"io/fs"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"

	"github.com/indium114/spyglass/lens"
)

type filesLens struct {
	home string

	mu    sync.RWMutex
	files []string

	indexing bool
}

func New() lens.Lens {
	home, _ := os.UserHomeDir()

	l := &filesLens{
		home: home,
	}

	l.loadCache()

	if home != "" {
		go l.index()
	}

	return l
}

func (l *filesLens) Name() string {
	return "Files"
}

func (l *filesLens) cachePath() string {
	cacheDir, err := os.UserCacheDir()
	if err != nil {
		return ""
	}
	return filepath.Join(cacheDir, "spyglass", "files", "index.json")
}

func (l *filesLens) loadCache() {
	path := l.cachePath()
	if path == "" {
		return
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return
	}

	var files []string
	if err := json.Unmarshal(data, &files); err != nil {
		return
	}

	l.mu.Lock()
	l.files = files
	l.mu.Unlock()
}

func (l *filesLens) saveCache(files []string) {
	path := l.cachePath()
	if path == "" {
		return
	}

	_ = os.MkdirAll(filepath.Dir(path), 0755)

	data, err := json.Marshal(files)
	if err != nil {
		return
	}

	_ = os.WriteFile(path, data, 0644)
}

func (l *filesLens) index() {
	l.mu.Lock()
	if l.indexing {
		l.mu.Unlock()
		return
	}
	l.indexing = true
	l.mu.Unlock()

	var newFiles []string

	filepath.WalkDir(l.home, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}

		if d.IsDir() && strings.HasPrefix(d.Name(), ".") && path != l.home {
			return filepath.SkipDir
		}

		newFiles = append(newFiles, path)
		return nil
	})

	l.mu.Lock()
	l.files = newFiles
	l.indexing = false
	l.mu.Unlock()

	go l.saveCache(newFiles)
}

func (l *filesLens) Search(query string) ([]lens.Entry, error) {
	l.mu.RLock()
	filesCopy := make([]string, len(l.files))
	copy(filesCopy, l.files)
	l.mu.RUnlock()

	query = strings.ToLower(strings.TrimSpace(query))

	var results []lens.Entry
	for _, path := range filesCopy {
		if query == "" || strings.Contains(strings.ToLower(path), query) {
			results = append(results, lens.Entry{
				ID:          path,
				Title:       shortenPath(l.home, path),
				Icon:        "󰈔",
				Description: path,
			})
		}
	}

	return results, nil
}

func (l *filesLens) Enter(e lens.Entry) error {
	cmd := exec.Command("xdg-open", e.ID)
	return cmd.Start()
}

func (l *filesLens) ContextActions(e lens.Entry) []lens.Action {
	return []lens.Action{
		{
			Name: "Reindex Files",
			Run: func(entry lens.Entry) error {
				go l.index()
				return nil
			},
		},
	}
}

func shortenPath(home, full string) string {
	if strings.HasPrefix(full, home) {
		full = "~" + strings.TrimPrefix(full, home)
	}

	sep := string(os.PathSeparator)
	hasLeadingSlash := strings.HasPrefix(full, sep)

	parts := strings.Split(full, sep)
	if len(parts) <= 2 {
		return full
	}

	for i := 0; i < len(parts)-1; i++ {
		part := parts[i]

		if part == "" || part == "~" {
			continue
		}

		if len(part) > 2 {
			parts[i] = part[:2]
		}
	}

	short := strings.Join(parts, sep)

	if hasLeadingSlash && !strings.HasPrefix(short, sep) {
		short = sep + short
	}

	return short
}
