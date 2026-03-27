package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/indium114/spyglass/lens"

	"github.com/charmbracelet/bubbles/textinput"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

type viewState int

const (
	stateEntries viewState = iota
	stateContext
)

type model struct {
	lenses     []lens.Lens
	activeLens int

	search textinput.Model

	entries  []lens.Entry
	selected int
	scroll   int
	state    viewState

	// Context menu fields
	actions         []lens.Action
	contextFor      lens.Entry
	contextSelected int
	contextScroll   int

	width  int
	height int

	// Cache entries for lazyloading
	loadedEntries map[int][]lens.Entry
}

func newModel() model {
	ti := textinput.New()
	ti.Placeholder = " Search..."
	ti.Focus()
	ti.CharLimit = 256

	m := model{
		lenses:        Lenses,
		search:        ti,
		loadedEntries: make(map[int][]lens.Entry),
		selected:      0,
		scroll:        0,
		state:         stateEntries,
	}
	m.refresh()
	return m
}

func (m *model) refresh() {
	if m.state == stateEntries {
		query := strings.TrimSpace(m.search.Value())

		entries, _ := m.lenses[m.activeLens].Search(query)

		if query == "" {
			entries, _ = m.lenses[m.activeLens].Search("")
		}

		m.entries = entries

		if m.selected >= len(m.entries) {
			m.selected = len(m.entries) - 1
		}
		if m.selected < 0 {
			m.selected = 0
		}
	}
}

func (m model) Init() tea.Cmd {
	return textinput.Blink
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {

	case tea.WindowSizeMsg:
		m.width = msg.Width
		m.height = msg.Height

	case tea.KeyMsg:
		switch msg.Type {

		case tea.KeyCtrlC:
			return m, tea.Quit

		case tea.KeyTab:
			// Switch lens
			m.activeLens = (m.activeLens + 1) % len(m.lenses)
			if m.activeLens < 0 {
				m.activeLens = len(m.lenses) - 1
			}
			m.state = stateEntries
			m.selected = 0
			m.scroll = 0
			m.refresh()

		case tea.KeyShiftTab:
			// Open context menu
			if m.state == stateEntries && len(m.entries) > 0 {
				entry := m.entries[m.selected]
				actions := m.lenses[m.activeLens].ContextActions(entry)

				if len(actions) > 0 {
					m.contextFor = entry
					m.actions = actions
					m.state = stateContext
					m.contextSelected = 0
					m.contextScroll = 0
				}
			}

		case tea.KeyUp:
			if m.state == stateEntries {
				if m.selected > 0 {
					m.selected--
				}
			} else if m.state == stateContext {
				if m.contextSelected > 0 {
					m.contextSelected--
				}
			}

		case tea.KeyDown:
			if m.state == stateEntries {
				if m.selected < len(m.entries)-1 {
					m.selected++
				}
			} else if m.state == stateContext {
				if m.contextSelected < len(m.actions)-1 {
					m.contextSelected++
				}
			}

		case tea.KeyEnter:
			if m.state == stateEntries && len(m.entries) > 0 {
				entry := m.entries[m.selected]
				m.lenses[m.activeLens].Enter(entry)
				return m, tea.Quit
			} else if m.state == stateContext && len(m.actions) > 0 {
				m.actions[m.contextSelected].Run(m.contextFor)
				m.state = stateEntries
				m.selected = 0
				m.scroll = 0
				return m, tea.Quit
			}

		case tea.KeyEsc:
			m.state = stateEntries
			m.selected = 0
			m.scroll = 0
		}
	}

	var cmd tea.Cmd
	m.search, cmd = m.search.Update(msg)
	m.refresh()
	return m, cmd
}

func (m model) View() string {
	if m.width <= 0 || m.height <= 0 {
		return ""
	}

	border := lipgloss.RoundedBorder()

	// Compute available space properly
	tabHeight := 3
	descHeight := 5
	searchHeight := 3

	listHeight := m.height - 1 - tabHeight - descHeight - searchHeight
	if listHeight < 3 {
		listHeight = 3
	}

	contentWidth := m.width - 2

	tabStyle := lipgloss.NewStyle().
		Border(border).
		BorderForeground(lipgloss.Color("#313244")).
		Width(contentWidth).
		Height(tabHeight-2).
		Padding(0, 1)

	listStyle := lipgloss.NewStyle().
		Border(border).
		BorderForeground(lipgloss.Color("#313244")).
		Width(contentWidth).
		Height(listHeight-2).
		Padding(0, 1)

	descStyle := lipgloss.NewStyle().
		Border(border).
		BorderForeground(lipgloss.Color("#313244")).
		Width(contentWidth).
		Height(descHeight-2).
		Padding(0, 1)

	searchStyle := lipgloss.NewStyle().
		Border(border).
		BorderForeground(lipgloss.Color("#cba6f7")).
		Width(contentWidth).
		Height(searchHeight-2).
		Padding(0, 1)

	// Tabs
	var tabs []string
	for i, l := range m.lenses {
		if i == m.activeLens {
			tabs = append(tabs, lipgloss.NewStyle().
				Foreground(lipgloss.Color("#cba6f7")).
				Bold(true).
				Render("["+l.Name()+"]"))
		} else {
			tabs = append(tabs, l.Name())
		}
	}
	tabsBox := tabStyle.Render(strings.Join(tabs, " | "))

	// LIST CONTENT
	var listBuilder strings.Builder

	maxVisible := listHeight - 2
	if maxVisible < 1 {
		maxVisible = 1
	}

	if m.state == stateContext {
		// Keep selected within visible window
		if m.contextSelected < m.contextScroll {
			m.contextScroll = m.contextSelected
		}
		if m.contextSelected >= m.contextScroll+maxVisible {
			m.contextScroll = m.contextSelected - maxVisible + 1
		}

		start := m.contextScroll
		end := start + maxVisible
		if end > len(m.actions) {
			end = len(m.actions)
		}

		for i := start; i < end; i++ {
			cursor := "  "
			if i == m.contextSelected {
				cursor = "> "
			}
			listBuilder.WriteString(cursor + m.actions[i].Name + "\n")
		}

	} else {
		if m.selected < m.scroll {
			m.scroll = m.selected
		}
		if m.selected >= m.scroll+maxVisible {
			m.scroll = m.selected - maxVisible + 1
		}

		start := m.scroll
		end := start + maxVisible
		if end > len(m.entries) {
			end = len(m.entries)
		}

		for i := start; i < end; i++ {
			cursor := "  "
			if i == m.selected {
				cursor = "> "
			}
			listBuilder.WriteString(
				fmt.Sprintf("%s%s %s\n",
					cursor,
					m.entries[i].Icon,
					m.entries[i].Title,
				),
			)
		}
	}

	listBox := listStyle.Render(listBuilder.String())

	// Description
	var desc string

	if m.state == stateEntries {
		if len(m.entries) > 0 && m.selected < len(m.entries) {
			desc = m.entries[m.selected].Description
		}
	} else if m.state == stateContext {
		if len(m.actions) > 0 && m.contextSelected < len(m.actions) {
			desc = "Action: " + m.actions[m.contextSelected].Name
		}
	}

	descBox := descStyle.Render(desc)

	searchBox := searchStyle.Render(m.search.View())

	return lipgloss.JoinVertical(
		lipgloss.Left,
		tabsBox,
		listBox,
		descBox,
		searchBox,
	)
}

func main() {
	p := tea.NewProgram(newModel())
	if err := p.Start(); err != nil {
		fmt.Println("Error:", err)
		os.Exit(1)
	}
}
