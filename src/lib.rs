pub trait Lens {
    fn name(&self) -> String;
    fn search(&self, query: String) -> Vec<Entry>;
}

pub struct Entry {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub description: String,
}
