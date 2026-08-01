use ratatui::text::Text;

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
    pub enter: fn(),
}

// impl<'a> From<&'a Entry> for Text<'a> {
//     fn from(entry: &'a Entry) -> Self {
//         Text::from(entry.title.clone())
//     }
// }
