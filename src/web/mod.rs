use serde::Deserialize;
use std::{
    fs,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use crate::{Entry, Lens};

#[derive(Deserialize)]
struct WebConfig {
    pub url: String,
}

fn load() -> WebConfig {
    let dir = dirs::config_dir()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
        + "/spyglass/web";

    let _ = fs::create_dir_all(dir.clone());

    let file = dir.clone() + "/config.toml";

    let contents = fs::read_to_string(file).expect("web# lens has not been configured yet.\nplease configure it in ~/.config/spyglass/web/config.toml");
    toml::from_str(&contents).expect("web# lens has not been configured yet or configuration is invalid.\nplease configure it in ~/.config/spyglass/web/config.toml")
}

fn enter(entry: &Entry) {
    let base_url = load().url;
    let query = url_escape::encode_fragment(&entry.meta);
    let url = base_url.replace("%s", &query);

    let mut cmd_binding = Command::new("xdg-open");
    let cmd = cmd_binding.arg(url);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    unsafe {
        cmd.pre_exec(|| {
            libc::setsid();
            Ok(())
        });
    }

    let _ = cmd.spawn();
}

pub struct Web;

impl Lens for Web {
    fn name(&self) -> String {
        "web".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        if !query.is_empty() {
            vec![Entry {
                id: "web_search".to_string(),
                title: query.clone(),
                icon: "".to_string(),
                meta: query.clone(),
                enter,
            }]
        } else {
            Vec::new()
        }
    }
}
