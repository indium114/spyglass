use rayon::prelude::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::Write,
    process::{Command, Stdio},
    sync::LazyLock,
};

use crate::{Entry, Lens};

static FETCH_URL: &str =
    "https://raw.githubusercontent.com/ryanoasis/nerd-fonts/refs/heads/master/glyphnames.json";

static MAX_RESULTS: usize = 50;
static GLYPHS: LazyLock<Vec<GlyphEntry>> = LazyLock::new(load_glyphs);

#[derive(Deserialize)]
struct GlyphValue {
    #[serde(default)]
    char: String,
    #[serde(default)]
    code: String,
}

struct GlyphEntry {
    name: String,
    char: String,
    code: String,
}

fn glyphs_path() -> String {
    dirs::cache_dir()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
        + "/spyglass/nerdfont/glyphnames.json"
}

fn download_glyphs() -> Result<(), Box<dyn std::error::Error>> {
    let dir = dirs::cache_dir()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
        + "/spyglass/nerdfont";

    let file = dir.clone() + "/glyphnames.json";

    let _ = fs::create_dir_all(dir.clone());

    if !fs::exists(file.clone()).unwrap_or(false) {
        let content = reqwest::blocking::get(FETCH_URL)?.bytes()?;
        fs::write(file.clone(), &content)?;
    }

    Ok(())
}

fn load_glyphs() -> Vec<GlyphEntry> {
    let _ = download_glyphs();
    let contents = fs::read_to_string(glyphs_path()).unwrap_or_default();
    let map: HashMap<String, GlyphValue> = serde_json::from_str(&contents).unwrap_or_default();

    map.into_par_iter()
        .filter(|(_, v)| !v.char.is_empty())
        .map(|(name, v)| GlyphEntry {
            name: name.to_lowercase(),
            char: v.char,
            code: v.code,
        })
        .collect()
}

fn copy(entry: &Entry) {
    let tool: &[&str] = if Command::new("wl-copy").stdin(Stdio::null()).spawn().is_ok() {
        &["wl-copy"]
    } else if Command::new("xclip").stdin(Stdio::null()).spawn().is_ok() {
        &["xclip", "-selection", "clipboard"]
    } else if Command::new("pbcopy").stdin(Stdio::null()).spawn().is_ok() {
        &["pbcopy"]
    } else {
        return;
    };

    let mut cmd = Command::new(tool[0])
        .args(&tool[1..])
        .stdin(Stdio::piped())
        .spawn()
        .expect("clipboard tool failed");
    if let Some(mut stdin) = cmd.stdin.take() {
        let _ = stdin.write_all(entry.icon.as_bytes());
    }
    let _ = cmd.wait();
}

pub struct NerdFont {}

impl Default for NerdFont {
    fn default() -> Self {
        Self::new()
    }
}

impl NerdFont {
    pub fn new() -> Self {
        let _ = download_glyphs();
        Self {}
    }
}

impl Lens for NerdFont {
    fn name(&self) -> String {
        "nerdfont".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        let q = query.to_lowercase();
        GLYPHS
            .iter()
            .filter(|g| q.is_empty() || g.name.contains(&q))
            .take(MAX_RESULTS)
            .map(|g| Entry {
                id: g.name.clone(),
                title: g.name.clone(),
                icon: g.char.clone(),
                meta: g.code.clone(),
                enter: copy,
            })
            .collect()
    }
}
