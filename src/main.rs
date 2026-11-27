use mlua::{Lua, LuaSerdeExt, Value};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use ratatui_image::picker::Picker;
use ratatui_textarea::TextArea;
use rayon::prelude::*;
use spyglass::{Entry, Lens};
use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
    thread,
    time::Duration,
    str::FromStr,
    fs,
};
use serde::Deserialize;

use crate::image_loader::ImageLoader;

mod image_loader;
mod lua_primitives;

fn home() -> String {
    let dir = dirs::home_dir();
    dir.map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

struct App {
    state: ListState,
    running: bool,
    query: String,
    results: Vec<Result>,
    image_loader: ImageLoader,
    popup: bool,
    config: Config,
}

struct Result {
    lens_name: String,
    entry: Entry,
}

#[derive(Clone, Deserialize)]
struct Config {
    pub search_icon_colour: String,
    pub search_box_border_colour: String,
    pub list_border_colour: String,
    pub list_highlight_colour: String,
    pub list_description_deselected_colour: String,
    pub lens_colour: String,
    pub image_workers: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        // MARK: load config
        let config_path = home() + "/.config/spyglass/globalconfig.toml";
        let contents = fs::read_to_string(config_path).expect("~/.config/spyglass/globalconfig.toml does not exist");
        let config: Config = toml::from_str(&contents).expect("~/.config/spyglass/globalconfig.toml is not valid toml");

        // decide image protocol based on terminal capabilities
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            state: list_state,
            running: true,
            query: "".to_string(),
            image_loader: ImageLoader::new(picker, config.image_workers.unwrap_or(8)),
            results: Vec::new(),
            popup: false,
            config: config.clone(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let mut lua = Lua::new();
        lua_primitives::primitives(&mut lua);

        // MARK: lenses
        let path = home() + "/.config/spyglass/lenses/";
        let lenses: Vec<Lens> = spyglass::load_lenses(&lua, &path);

        if self.state.selected().is_none() {
            self.state.select(Some(0));
        }

        thread::sleep(Duration::from_millis(80));
        while ratatui::crossterm::terminal::size()
            .map(|(w, h)| w == 0 || h == 0)
            .unwrap_or(true)
        {
            thread::sleep(Duration::from_millis(10));
        }

        let mut textarea = TextArea::default();
        let mut needs_redraw = true;
        while self.running {
            if self.image_loader.poll() {
                needs_redraw = true;
            }

            if needs_redraw {
                terminal.draw(|frame| {
                    self.render(frame, &mut textarea, &lenses, &lua);
                })?;
                needs_redraw = false;
            }

            if self.keybinds(&mut textarea)? {
                needs_redraw = true;
            }
        }

        Ok(())
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        textarea: &mut TextArea,
        lenses: &Vec<Lens>,
        lua: &Lua,
    ) {
        // MARK: searching
        self.results = Vec::new();
        if self.query.contains('#') {
            let split = self.query.split('#').collect::<Vec<&str>>();
            let lens_filter: String = split[0].to_string();
            let query = split[1].trim().to_string();

            for lens in lenses {
                if lens.name == lens_filter {
                    let value: Value = lens.search.call(query.clone()).unwrap();
                    let entries: Vec<Entry> = lua.from_value(value).unwrap();
                    for entry in entries {
                        self.results.push(Result {
                            lens_name: lens.name.clone(),
                            entry,
                        });
                    }
                }
            }
        } else {
            for lens in lenses {
                let value: Value = lens.search.call(self.query.clone()).unwrap();
                let entries: Vec<Entry> = lua.from_value(value).unwrap();

                for entry in entries {
                    self.results.push(Result {
                        lens_name: lens.name.clone(),
                        entry,
                    });
                }
            }
        }

        if self.state.selected().is_none() {
            self.state.select(Some(0));
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
                    .border_style(Style::default().fg(Color::from_str(&self.config.search_icon_colour).unwrap()))
                    .border_type(BorderType::Rounded),
            ),
            topbar_layout[0],
        );

        // MARK: textarea
        textarea.set_block(
            Block::new()
                .title_bottom(
                    Line::from(
                        "<".to_string()
                            + &(self.state.selected().unwrap_or(0) + 1).to_string()
                            + "/"
                            + &self.results.len().to_string()
                            + ">",
                    )
                    .right_aligned(),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::from_str(&self.config.search_box_border_colour).unwrap())),
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
                .par_iter()
                .map(|n| {
                    let icon_span = if n.entry.image.is_some() {
                        Span::raw(" ")
                    } else {
                        Span::raw(n.entry.icon.clone())
                    };
                    ListItem::new(Line::from(vec![
                        Span::styled(
                            n.lens_name.clone() + "#",
                            Style::default().fg(Color::from_str(&self.config.lens_colour).unwrap()),
                        ),
                        Span::raw(" "),
                        icon_span,
                        Span::raw(" "),
                        Span::raw(n.entry.title.clone()),
                        Span::raw(" "),
                        Span::styled(
                            " | ".to_string() + &n.entry.description.clone(),
                            Style::default().fg(Color::from_str(&self.config.list_description_deselected_colour).unwrap()),
                        ),
                    ]))
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(Color::from_str(&self.config.list_border_colour).unwrap()))
                .border_type(BorderType::Rounded),
        )
        .highlight_symbol("▌")
        .highlight_style(Style::new().fg(Color::from_str(&self.config.list_highlight_colour).unwrap()));
        frame.render_stateful_widget(list, master_layout[1], &mut self.state);

        // receive all loaded images from loading thread
        // replace all image-containing entries' icon placeholders with images
        for (row, n) in self
            .results
            .iter_mut()
            .skip(self.state.offset())
            .take(master_layout[1].height as usize)
            .enumerate()
        {
            if let Some(image_path) = &n.entry.image
                && let Some(protocol) = self.image_loader.get_or_queue(image_path)
            {
                let x = master_layout[1].x + n.lens_name.chars().count() as u16 + 4;
                let y = master_layout[1].y + 1 + row as u16;
                frame.render_widget(ratatui_image::Image::new(protocol), Rect::new(x, y, 2, 1));
            }
        }

        // MARK: lens list popup
        if self.popup {
            let lens_names: Vec<String> = lenses.iter().map(|l| l.name.clone()).collect();
            let lens_lines: Vec<Line> = lens_names
                .par_iter()
                .map(|n| {
                    Line::from(Span::styled(
                        n.to_owned() + "#",
                        Style::default().fg(Color::Yellow),
                    ))
                })
                .collect();

            let popup_area = frame
                .area()
                .centered(Constraint::Percentage(50), Constraint::Percentage(70));
            frame.render_widget(Clear, popup_area);
            frame.render_widget(
                Paragraph::new(Text::from(lens_lines)).block(
                    Block::default()
                        .title_top(Line::from(" Available lenses ").left_aligned())
                        .title_top(
                            Line::from(" v".to_string() + env!("CARGO_PKG_VERSION") + " ")
                                .right_aligned(),
                        )
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::from_str(&"#cba6f7").unwrap()))
                        .border_type(BorderType::Rounded),
                ),
                popup_area,
            );
        }
    }

    fn keybinds(&mut self, textarea: &mut TextArea) -> std::io::Result<bool> {
        if !event::poll(Duration::from_millis(16))? {
            return Ok(false);
        }
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Esc if !self.popup => self.running = false,
                    KeyCode::Up => self.state.select_previous(),
                    KeyCode::Down => self.state.select_next(),
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected()
                            && let Some(result) = self.results.get(i)
                        {
                            let mut cmd = Command::new("sh");
                            cmd.arg("-c")
                                .arg(result.entry.enter.clone())
                                .stdin(Stdio::null())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .process_group(0);
                            let _ = cmd.spawn();
                            self.running = false;
                        }
                    }
                    KeyCode::Tab => {
                        if let Some(i) = self.state.selected()
                            && let Some(result) = self.results.get(i)
                            && let Some(alternate) = &(result.entry.alternate)
                        {
                            let mut cmd = Command::new("sh");
                            cmd.arg("-c")
                                .arg(alternate)
                                .stdin(Stdio::null())
                                .stdout(Stdio::null())
                                .stderr(Stdio::null())
                                .process_group(0);
                            let _ = cmd.spawn();
                            self.running = false;
                        }
                    }
                    KeyCode::Char('?') => self.popup = true,
                    KeyCode::Esc if self.popup => self.popup = false,
                    _ => {
                        textarea.input(key_event);
                        self.state.select(Some(0));
                    }
                }
            }
            _ => (),
        }

        Ok(true)
    }
}

fn main() -> std::io::Result<()> {
    let _ = color_eyre::install();
    ratatui::run(|terminal| App::new().run(terminal))
}
