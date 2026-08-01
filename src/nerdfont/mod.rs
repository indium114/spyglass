use std::{fs, time::Duration};

use crate::{Entry, Lens};

static FETCH_URL: &'static str =
    "https://raw.githubusercontent.com/ryanoasis/nerd-fonts/refs/heads/master/glyphnames.json";

struct GlyphEntry {
    name: String,
    char: String,
    code: String,
}

// PERF: fix whatever is slowing this down
fn download_glyphs() -> Result<(), Box<dyn std::error::Error>> {
    let dir = dirs::cache_dir()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
        + "/spyglass/nerdfont/glyphnames.json";

    if !fs::exists(dir.clone()).unwrap_or(false) {
        let content = reqwest::blocking::get(FETCH_URL)?.bytes()?;
        fs::write(dir.clone(), &content)?;
    }

    Ok(())
}

pub struct NerdFont {
    glyphs: Vec<GlyphEntry>,
}

impl NerdFont {
    pub fn new() -> Self {
        let _ = download_glyphs();
        Self { glyphs: Vec::new() }
    }
}

impl Lens for NerdFont {
    fn name(&self) -> String {
        "nerdfont".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        if query != "".to_string() {
            let _ = download_glyphs();
            let entries: Vec<Entry> = Vec::new();

            entries
        } else {
            Vec::new()
        }
    }
}
