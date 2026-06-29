use crate::{applications::ApplicationsLens, lens::Lens};

pub fn lenses() -> Vec<Box<dyn Lens>> {
    vec![Box::new(ApplicationsLens::new())]
}
