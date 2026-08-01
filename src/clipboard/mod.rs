use std::{os::unix::process::CommandExt, process::Command};

use crate::{Entry, Lens};

fn truncate(s: &String) -> String {
    let max = 20;
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let byte_idx = s
            .char_indices()
            .nth(max)
            .map(|(idx, _)| idx)
            .unwrap_or(s.len());

        format!("{}...", &s[..byte_idx])
    }
}

fn copy(entry: &Entry) {
    let mut cmd_bind = Command::new("wl-copy");
    let cmd = cmd_bind.arg(entry.meta.clone());
    let _ = cmd.exec();
}

pub struct Clipboard;

impl Lens for Clipboard {
    fn name(&self) -> String {
        "clipboard".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        let output = Command::new("cliphist")
            .arg("list")
            .output()
            .expect("failed to run cliphist");
        let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string().split('\t').nth(1).unwrap().to_string())
            .collect();

        let mut entries: Vec<Entry> = Vec::new();
        for line in lines {
            if line.to_lowercase().contains(&query) {
                entries.push(Entry {
                    id: line.clone(),
                    title: truncate(&line),
                    icon: "".to_string(),
                    meta: line.clone(),
                    enter: copy,
                });
            }
        }

        entries
    }
}
