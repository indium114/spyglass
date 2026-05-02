package applications

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"syscall"
	"time"

	"github.com/indium114/spyglass/lens"

	"gopkg.in/yaml.v3"
)

type appConfig struct {
	Name        string `yaml:"name"`
	Icon        string `yaml:"icon"`
	Command     string `yaml:"command"`
	Description string `yaml:"description"`
	Context     []struct {
		Name    string `yaml:"name"`
		Command string `yaml:"command"`
	} `yaml:"context"`
}

type applicationsLens struct {
	apps []appConfig
}

func New() lens.Lens {
	l := &applicationsLens{}
	l.load()
	return l
}

func (a *applicationsLens) Name() string {
	return "Applications"
}

func (a *applicationsLens) load() {
	home, _ := os.UserHomeDir()
	dir := filepath.Join(home, ".config", "spyglass", "applications")
	files, _ := os.ReadDir(dir)

	for _, f := range files {
		if strings.HasSuffix(f.Name(), ".yaml") {
			data, _ := os.ReadFile(filepath.Join(dir, f.Name()))
			var cfg appConfig
			yaml.Unmarshal(data, &cfg)
			a.apps = append(a.apps, cfg)
		}
	}
}

func (a *applicationsLens) Search(query string) ([]lens.Entry, error) {
	var results []lens.Entry
	query = strings.ToLower(query)

	for _, app := range a.apps {
		if strings.Contains(strings.ToLower(app.Name), query) {
			results = append(results, lens.Entry{
				ID:          app.Name,
				Title:       app.Name,
				Icon:        app.Icon,
				Description: app.Description,
			})
		}
	}
	return results, nil
}

func (a *applicationsLens) Enter(entry lens.Entry) error {
	for _, app := range a.apps {
		if app.Name == entry.Title {
			cmd := exec.Command("sh", "-c", app.Command)
			cmd.Stdout = nil
			cmd.Stdin = nil
			cmd.Stderr = nil
			cmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

			if err := cmd.Start(); err != nil {
				return err
			}

			// ensure that the process has started before returning
			timeout := 10 * time.Second
			ticker := time.NewTicker(50 * time.Millisecond)
			defer ticker.Stop()
			deadline := time.After(timeout)

			for {
				select {
				case <-ticker.C:
					// check if process exists using signal 0
					if err := cmd.Process.Signal(syscall.Signal(0)); err == nil {
						return nil
					}
				case <-deadline:
					if err := cmd.Process.Signal(syscall.Signal(0)); err == nil {
						return nil
					}
					return fmt.Errorf("Application %q failed to start within %v", app.Name, timeout)
				}
			}
		}
	}
	return nil
}

func (a *applicationsLens) ContextActions(entry lens.Entry) []lens.Action {
	for _, app := range a.apps {
		if app.Name == entry.Title {
			var actions []lens.Action
			for _, c := range app.Context {
				command := c.Command
				actions = append(actions, lens.Action{
					Name: c.Name,
					Run: func(e lens.Entry) error {
						cmd := exec.Command("sh", "-c", command)
						cmd.Stdout = nil
						cmd.Stdin = nil
						cmd.Stderr = nil
						cmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

						if err := cmd.Start(); err != nil {
							return err
						}

						// ensure that the process has started before returning
						timeout := 10 * time.Second
						ticker := time.NewTicker(50 * time.Millisecond)
						defer ticker.Stop()
						deadline := time.After(timeout)

						for {
							select {
							case <-ticker.C:
								// check if process exists using signal 0
								if err := cmd.Process.Signal(syscall.Signal(0)); err == nil {
									return nil
								}
							case <-deadline:
								if err := cmd.Process.Signal(syscall.Signal(0)); err == nil {
									return nil
								}
								return fmt.Errorf("Action %q failed to start within %v", c.Name, timeout)
							}
						}
					},
				})
			}
			return actions
		}
	}
	return nil
}
