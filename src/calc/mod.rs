use exp_rs::interp;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::{Entry, Lens};

fn enter(entry: &Entry) {
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

fn calculate(expr: &str) -> String {
    interp(expr, None).unwrap_or(0.0).to_string()
}

pub struct Calc;

impl Lens for Calc {
    fn name(&self) -> String {
        "calc".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        if !query.is_empty() {
            let result = calculate(&query);
            vec![Entry {
                id: "calc_result".to_string(),
                title: result.clone(),
                icon: "".to_string(),
                meta: result.clone(),
                enter,
            }]
        } else {
            Vec::new()
        }
    }
}
