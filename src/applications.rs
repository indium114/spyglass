use crate::lens::{Entry, Lens};
use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};
use serde::{Deserialize, Serialize};
use std::{fs, io};

// helper
fn home() -> String {
    let dir = dirs::home_dir();
    dir.map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
}

#[derive(Serialize, Deserialize)]
struct AppContext {
    name: String,
    command: String,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    name: String,
    icon: String,
    command: String,
    description: String,
    context: Vec<AppContext>,
}

pub struct ApplicationsLens {
    pub apps: Vec<AppConfig>,
}

impl ApplicationsLens {
    pub fn new() -> Self {
        Self { apps: Vec::new() }
    }

    pub fn load(&mut self) {
        let home = home();
        let dir = home + ".config/spyglass/applications";
        let files = fs::read_dir(dir).expect("failed to read ~/.config/spyglass/applications");

        for i in files {
            let i = i.unwrap();
            let config = fs::read_to_string(i.path())
                .ok()
                .and_then(|s| serde_yaml::from_str(&s).ok())
                .unwrap();

            self.apps.push(config);
        }
    }
}

impl Lens for ApplicationsLens {
    fn name(&self) -> &str {
        "Applications"
    }

    fn search(&self, query: &str) -> Result<Vec<crate::lens::Entry>, color_eyre::eyre::Error> {
        let mut results: Vec<Entry> = Vec::new();
        let matcher = SkimMatcherV2::default();
        for app in &*self.apps {
            if let Some(score) = matcher.fuzzy_match(&app.name, query) {
                results.push(Entry {
                    id: app.name.to_string(),
                    title: app.name.to_string(),
                    icon: app.icon.to_string(),
                    description: app.description.to_string(),
                });
            }
        }
        Ok(results)
    }

    fn enter(&self, entry: &crate::lens::Entry) -> Result<(), color_eyre::eyre::Error> {
        todo!()
    }

    fn context_actions(&self, entry: &crate::lens::Entry) -> Vec<crate::lens::Action> {
        Vec::new()
    }
}
