use serde::Deserialize;
use std::{
    fs,
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use crate::{Entry, Lens};

#[derive(Deserialize)]
struct AppConfig {
    pub name: String,
    pub icon: String,
    pub command: String,
}

fn load() -> Vec<AppConfig> {
    let mut apps: Vec<AppConfig> = Vec::new();
    let dir = dirs::config_dir()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
        + "/spyglass/applications/";

    let _ = fs::create_dir_all(dir.clone());

    for file in fs::read_dir(dir).unwrap() {
        let file = file.unwrap().path();
        if file.extension().is_some_and(|ext| ext == "toml") {
            let contents = fs::read_to_string(file).unwrap();
            let parsed: AppConfig = toml::from_str(&contents).unwrap();
            apps.push(parsed);
        }
    }

    apps
}

fn enter(entry: &Entry) {
    let mut cmd_binding = Command::new("sh");
    let cmd = cmd_binding.arg("-c").arg(entry.meta.clone());
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

pub struct Apps;

impl Lens for Apps {
    fn name(&self) -> String {
        "apps".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        let mut results: Vec<Entry> = Vec::new();
        let query = query.to_lowercase();
        for app in load() {
            if app.name.to_lowercase().contains(&query) {
                results.push(Entry {
                    id: app.name.clone(),
                    title: app.name.clone(),
                    icon: app.icon,
                    meta: app.command,
                    enter,
                });
            }
        }

        results
    }
}
