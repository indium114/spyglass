use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use crate::{Entry, Lens};

fn enter(entry: &Entry) {
    let mut cmd_binding = Command::new("systemctl");
    let cmd = cmd_binding.arg(entry.meta.clone());
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

pub struct Power;

impl Lens for Power {
    fn name(&self) -> String {
        "power".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        let entries: Vec<Entry> = vec![
            Entry {
                id: "poweroff".to_string(),
                title: "Shutdown".to_string(),
                icon: "⏻".to_string(),
                meta: "poweroff".to_string(),
                enter,
            },
            Entry {
                id: "reboot".to_string(),
                title: "Reboot".to_string(),
                icon: "".to_string(),
                meta: "reboot".to_string(),
                enter,
            },
            Entry {
                id: "suspend".to_string(),
                title: "Suspend".to_string(),
                icon: "⏾".to_string(),
                meta: "suspend".to_string(),
                enter,
            },
        ];

        let mut final_entries: Vec<Entry> = Vec::new();
        for entry in entries {
            if entry.title.to_lowercase().contains(&query) {
                final_entries.push(entry);
            }
        }

        final_entries
    }
}
