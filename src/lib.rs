use image::ImageReader;
use mlua::{Function, Lua, Table};
use ratatui::layout::Size;
use ratatui_image::{Resize, picker::Picker, protocol::Protocol};
use serde::Deserialize;
use std::{fs, path::PathBuf};

mod builtins;

pub struct Lens {
    pub name: String,
    pub search: Function,
}

#[derive(Clone, Deserialize)]
pub struct Entry {
    /// unique internal id for each entry
    pub id: String,
    /// user-facing, 'pretty' title
    pub title: String,
    /// entry description
    pub description: String,
    /// single-character icon for the entry
    pub icon: String,
    /// extra info, typically used if `enter()` needs more info
    pub meta: String,
    /// function to run when the user presses _Enter_ on the entry
    pub enter: String,
    /// alternate function to run when the user presses _Tab_ on the entry
    pub alternate: Option<String>,
    /// path to image
    pub image: Option<String>,
}

pub fn load_lenses(lua: &Lua, dir: &str) -> Vec<Lens> {
    let mut lenses: Vec<Lens> = Vec::new();
    lenses.extend(builtins::init(lua));

    if std::path::Path::new(dir).is_dir() {
        for item in fs::read_dir(dir).unwrap_or_else(|_| panic!("{dir} does not exist")) {
            let path = item.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("lua") {
                continue;
            }

            let src = fs::read_to_string(&path).unwrap();
            let table: Table = lua
                .load(&src)
                .set_name(path.to_string_lossy())
                .eval()
                .unwrap();

            lenses.push(Lens {
                name: table.get("name").unwrap(),
                search: table.get("search").unwrap(),
            })
        }
    }

    lenses
}

pub fn load_image_protocol(path: &PathBuf, picker: &Picker) -> Option<Protocol> {
    let reader = ImageReader::open(path).ok()?.with_guessed_format().ok()?;
    let dyn_img = reader.decode().ok()?;
    match picker.new_protocol(dyn_img, Size::new(2, 1), Resize::Fit(None)) {
        Ok(proto) => Some(proto),
        Err(e) => {
            eprintln!("Failed to create protocol: {:?}", e);
            None
        }
    }
}
