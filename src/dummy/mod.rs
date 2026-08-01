use crate::{Entry, Lens};

pub struct Dummy;

fn function() {}

impl Lens for Dummy {
    fn name(&self) -> String {
        "dummy".to_string()
    }

    fn search(&self, query: String) -> Vec<Entry> {
        let mut entries: Vec<Entry> = Vec::new();
        for i in 1..=(query.len() as u64) {
            entries.push(Entry {
                id: "entry no.".to_string() + &i.to_string(),
                title: "entry no. ".to_string() + &i.to_string(),
                icon: "".to_string(),
                enter: function,
            });
        }

        entries
    }
}
