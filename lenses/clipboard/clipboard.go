package clipboard

import (
	"bufio"
	"bytes"
	"os/exec"
	"strings"
	"syscall"

	"github.com/indium114/spyglass/lens"
)

type clipboardLens struct{}

func New() lens.Lens {
	return &clipboardLens{}
}

func (l *clipboardLens) Name() string {
	return "Clipboard"
}

func (l *clipboardLens) Search(query string) ([]lens.Entry, error) {
	cmd := exec.Command("cliphist", "list")

	out, err := cmd.Output()
	if err != nil {
		return nil, err
	}

	var entries []lens.Entry
	scanner := bufio.NewScanner(bytes.NewReader(out))

	for scanner.Scan() {
		line := scanner.Text()

		// parse out the clipboard entry ID
		parts := strings.SplitN(line, "\t", 2)
		if len(parts) != 2 {
			continue
		}

		id := parts[0]
		text := parts[1]

		if query == "" || strings.Contains(strings.ToLower(text), strings.ToLower(query)) {
			entries = append(entries, lens.Entry{
				ID:          id,
				Title:       text,
				Icon:        "",
				Description: "Entry ID " + id,
			})
		}
	}

	return entries, nil
}

func (l *clipboardLens) Enter(e lens.Entry) error {
	cmd := exec.Command("cliphist", "decode", e.ID)

	copyCmd := exec.Command("wl-copy")
	copyCmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

	pipe, err := cmd.StdoutPipe()
	if err != nil {
		return err
	}

	copyCmd.Stdin = pipe

	if err := cmd.Start(); err != nil {
		return err
	}

	if err := copyCmd.Start(); err != nil {
		return err
	}

	return nil
}

func (l *clipboardLens) ContextActions(e lens.Entry) []lens.Action {
	return []lens.Action{
		{
			Name: "Copy to Clipboard",
			Run: func(entry lens.Entry) error {
				cmd := exec.Command("cliphist", "decode", entry.ID)
				copyCmd := exec.Command("wl-copy")
				copyCmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

				pipe, err := cmd.StdoutPipe()
				if err != nil {
					return err
				}

				copyCmd.Stdin = pipe

				if err := cmd.Start(); err != nil {
					return err
				}

				return copyCmd.Start()
			},
		},
		{
			Name: "Clear History",
			Run: func(entry lens.Entry) error {
				cmd := exec.Command("cliphist", "wipe")
				return cmd.Run()
			},
		},
	}
}
