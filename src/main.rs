use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    event::KeyCode::Esc => break,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    Ok(())
}
fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("[Lens 1] | Lens 2 | Lens 3 | Lens 4").block(Block::new().borders(Borders::ALL)),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("Result 1\nResult 2\nResult 3\nResult 4\nResult 5").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
    frame.render_widget(
        Paragraph::new("This is a description. The quick brown fox jumps over the lazy dog").block(Block::new().borders(Borders::ALL)),
        layout[2],
    );
    frame.render_widget(
        Paragraph::new("> Search").block(Block::new().borders(Borders::ALL)),
        layout[3],
    );
}
