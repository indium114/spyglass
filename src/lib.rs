pub mod dummy;

pub trait Lens {
    fn name(&self) -> String;
    fn search(&self, query: String) -> Vec<Entry>;
}

#[derive(Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub enter: fn(&Entry),
}
