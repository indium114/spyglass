use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, ListState},
};
use ratatui_textarea::TextArea;

struct App {
    state: ListState,
    running: bool,
    query: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            running: true,
            query: "".to_string(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let mut textarea = TextArea::default();
        while self.running {
            terminal.draw(|frame| {
                self.render(frame, &mut textarea);
            })?;
            self.keybinds(&mut textarea)?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, textarea: &mut TextArea) {
        let master_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(10), Constraint::Fill(1)])
            .split(frame.area());

        let search_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(master_layout[1]);

        // MARK: textarea
        textarea.set_block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::Rgb(203, 166, 247))),
        );
        textarea.set_placeholder_text(" Search...");
        self.query = textarea
            .lines()
            .join(" ")
            .trim_end()
            .to_lowercase()
            .to_string();

        frame.render_widget(&*textarea, search_layout[0]);
    }

    fn keybinds(&mut self, textarea: &mut TextArea) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => self.running = false,
                    _ => {
                        textarea.input(key_event);
                    }
                }
            }
            _ => (),
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}
