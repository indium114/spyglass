use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};
use ratatui_textarea::TextArea;
use spyglass::{Entry, Lens, apps::Apps, power::Power, web::Web};

struct App {
    state: ListState,
    running: bool,
    query: String,
    results: Vec<Result>,
}

struct Result {
    lens_name: String,
    entry: Entry,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            state: list_state,
            running: true,
            query: "".to_string(),
            results: Vec::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        // MARK: lenses
        // NOTE: add lenses to vector below VVVVVV
        let lenses: Vec<Box<dyn Lens>> = vec![Box::new(Apps), Box::new(Power), Box::new(Web)];

        if self.state.selected().is_none() {
            self.state.select(Some(0));
        }

        let mut textarea = TextArea::default();
        while self.running {
            terminal.draw(|frame| {
                self.render(frame, &mut textarea, &lenses);
            })?;
            self.keybinds(&mut textarea)?;
            terminal.draw(|frame| {
                self.render(frame, &mut textarea, &lenses);
            })?;
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, textarea: &mut TextArea, lenses: &Vec<Box<dyn Lens>>) {
        // MARK: searching
        self.results = Vec::new();
        if self.query.contains('#') {
            let split = self.query.split('#').collect::<Vec<&str>>();
            let lens_filter: String = split[0].to_string();
            let query = split[1].trim().to_string();

            for lens in lenses {
                if lens.name() == lens_filter {
                    for entry in lens.search(query.clone()) {
                        self.results.push(Result {
                            lens_name: lens.name(),
                            entry: entry,
                        });
                    }
                }
            }
        } else {
            for lens in lenses {
                for entry in lens.search(self.query.trim().to_string()) {
                    self.results.push(Result {
                        lens_name: lens.name(),
                        entry: entry,
                    });
                }
            }
        }

        // MARK: rendering
        let master_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(frame.area());
        let topbar_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(6), Constraint::Fill(1)])
            .split(master_layout[0]);

        frame.render_widget(
            Paragraph::new("   ").block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(203, 166, 247)))
                    .border_type(BorderType::Rounded),
            ),
            topbar_layout[0],
        );

        // MARK: textarea
        textarea.set_block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::Rgb(203, 166, 247))),
        );
        textarea.set_placeholder_text("Search...");
        self.query = textarea
            .lines()
            .join(" ")
            .trim_end()
            .to_lowercase()
            .to_string();

        frame.render_widget(&*textarea, topbar_layout[1]);

        // MARK: list
        let list = List::new(
            self.results
                .iter()
                .map(|n| {
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            n.lens_name.clone() + "#",
                            Style::default().fg(Color::Yellow),
                        ),
                        Span::raw(" "),
                        Span::raw(n.entry.icon.clone()),
                        Span::raw(" "),
                        Span::raw(n.entry.title.clone()),
                    ]))
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::Rgb(203, 166, 247)))
                .border_type(BorderType::Rounded),
        )
        .highlight_symbol("|")
        .highlight_style(Style::new().fg(Color::Rgb(203, 166, 247)));
        frame.render_stateful_widget(list, master_layout[1], &mut self.state);
    }

    fn keybinds(&mut self, textarea: &mut TextArea) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc => self.running = false,
                    KeyCode::Up => self.state.select_previous(),
                    KeyCode::Down => self.state.select_next(),
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected() {
                            if let Some(result) = self.results.get(i) {
                                (result.entry.enter)(&result.entry);
                                self.running = false;
                            }
                        }
                    }
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
