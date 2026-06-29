use color_eyre::eyre::Error;

pub struct Entry {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub description: String,
}

pub struct Action {
    pub name: String,
    pub run: Box<dyn Fn(&Entry) -> Result<(), Error>>,
}

pub trait Lens {
    fn name(&self) -> &str;
    fn search(&self, query: &str) -> Result<Vec<Entry>, Error>;
    fn enter(&self, entry: &Entry) -> Result<(), Error>;
    fn context_actions(&self, entry: &Entry) -> Vec<Action>;
}
