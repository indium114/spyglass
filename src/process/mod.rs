use std::{collections::HashMap, sync::LazyLock};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

use crate::{Entry, Lens};

static PROCESSES: LazyLock<HashMap<Pid, String>> = LazyLock::new(load_procs);

fn load_procs() -> HashMap<Pid, String> {
    let mut system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );
    system.refresh_all();

    let mut procs: HashMap<Pid, String> = HashMap::new();

    for (pid, proc) in system.processes() {
        procs.insert(pid.clone(), proc.name().to_string_lossy().into_owned());
    }

    procs
}

fn enter(entry: &Entry) {
    let mut sys = System::new();
    sys.refresh_all();

    if let Some(proc) = sys.process(Pid::from_u32(entry.meta.parse::<u32>().unwrap())) {
        let _ = proc.kill();
    }
}

pub struct Procs;

impl Lens for Procs {
    fn name(&self) -> String {
        "process".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        let mut entries: Vec<Entry> = Vec::new();
        for (pid, name) in PROCESSES.iter() {
            if name.to_lowercase().contains(&query) {
                entries.push(Entry {
                    id: pid.to_string(),
                    title: pid.to_string() + &name,
                    icon: "".to_string(),
                    meta: pid.to_string(),
                    enter,
                });
            }
        }

        if query.is_empty() {
            entries.truncate(50);
            return entries;
        }

        entries
    }
}
