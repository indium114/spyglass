pub mod apps;
pub mod clipboard;
pub mod nerdfont;
pub mod power;
pub mod web;

pub trait Lens {
    /// returns the lens's name, conventionally all-lowercase
    fn name(&self) -> String;
    /// returns a vec of entries that match the given `query`.
    /// should `query` be empty, it should return _all_ results.
    fn search(&self, query: String) -> Vec<Entry>;
}

#[derive(Clone)]
pub struct Entry {
    /// unique internal id for each entry
    pub id: String,
    /// user-facing, 'pretty' title
    pub title: String,
    /// single-character icon for the entry
    pub icon: String,
    /// extra info, typically used if `enter()` needs more info
    pub meta: String,
    /// function to run when the user presses _Enter_ on the entry
    pub enter: fn(&Entry),
}
