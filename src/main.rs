use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
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
    frame.render_widget("Hello, world!", frame.area());
}
