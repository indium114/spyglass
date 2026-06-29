use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph},
};

#[derive(Debug, Default)]
pub struct App {
    items: Vec<String>,
    state: ListState,
}

fn main() -> color_eyre::Result<()> {
    let mut app = App::new(vec![
        "Item 1".to_string(),
        "Item 2".to_string(),
        "Item 3".to_string(),
        "Item 4".to_string(),
        "Item 5".to_string(),
        "Item 6".to_string(),
        "Item 7".to_string(),
        "Item 8".to_string(),
        "Item 9".to_string(),
        "Item 10".to_string(),
        "Item 11".to_string(),
        "Item 12".to_string(),
        "Item 13".to_string(),
        "Item 14".to_string(),
        "Item 15".to_string(),
        "Item 16".to_string(),
        "Item 17".to_string(),
        "Item 18".to_string(),
        "Item 19".to_string(),
        "Item 20".to_string(),
        "Item 21".to_string(),
        "Item 22".to_string(),
        "Item 23".to_string(),
        "Item 24".to_string(),
        "Item 25".to_string(),
        "Item 26".to_string(),
        "Item 27".to_string(),
        "Item 28".to_string(),
    ]);
    color_eyre::install()?;
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}

impl App {
    pub fn new(items: Vec<String>) -> Self {
        Self {
            items,
            state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn select_prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        event::KeyCode::Esc => break,
                        event::KeyCode::Up => self.select_prev(),
                        event::KeyCode::Down => self.select_next(),
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let list = List::new(self.items.clone())
            .highlight_style(Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        // MARK: layout
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Length(3),
            ])
            .split(frame.area());

        // MARK: tab bar
        frame.render_widget(
            Paragraph::new("[Lens 1] | Lens 2 | Lens 3 | Lens 4").block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            layout[0],
        );

        // MARK: list
        frame.render_stateful_widget(
            list.block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            layout[1],
            &mut self.state,
        );

        // MARK: description
        frame.render_widget(
            Paragraph::new("This is a description. The quick brown fox jumps over the lazy dog")
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                ),
            layout[2],
        );

        // MARK: search bar
        frame.render_widget(
            Paragraph::new("> Search").block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            layout[3],
        );
    }
}
