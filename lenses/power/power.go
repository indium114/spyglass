package power

import (
	"os/exec"
	"syscall"

	"github.com/indium114/spyglass/lens"
)

type powerLens struct{}

func New() lens.Lens {
	return &powerLens{}
}

func (p *powerLens) Name() string {
	return "Power"
}

func (p *powerLens) Search(query string) ([]lens.Entry, error) {
	return []lens.Entry{
		{
			ID:          "shutdown",
			Title:       "Shutdown",
			Icon:        "⏻",
			Description: "Power off the system",
		},
		{
			ID:          "reboot",
			Title:       "Reboot",
			Icon:        "",
			Description: "Restart the system",
		},
		{
			ID:          "suspend",
			Title:       "Suspend",
			Icon:        "⏾",
			Description: "Suspend to RAM",
		},
	}, nil
}

func (p *powerLens) Enter(entry lens.Entry) error {
	return runPowerCommand(entry.ID)
}

func (p *powerLens) ContextActions(entry lens.Entry) []lens.Action {
	// No context menu
	return nil
}

func runPowerCommand(id string) error {
	var cmd *exec.Cmd

	switch id {
	case "shutdown":
		cmd = exec.Command("systemctl", "poweroff")
	case "reboot":
		cmd = exec.Command("systemctl", "reboot")
	case "suspend":
		cmd = exec.Command("systemctl", "suspend")
	default:
		return nil
	}

	// Detach from Spyglass so it doesn't wait
	cmd.Stdout = nil
	cmd.Stderr = nil
	cmd.Stdin = nil
	cmd.SysProcAttr = &syscall.SysProcAttr{Setsid: true}

	return cmd.Start()
}
