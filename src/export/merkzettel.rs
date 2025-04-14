use crate::infrastructure::templates::Templates;
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
pub struct Merkzettel {
    pub items: Vec<Item>,
    pub no_google_fonts: bool
}

#[derive(Serialize)]
pub struct Item {
    pub titel: String,
    pub autor: Option<String>,
    pub preis: Option<String>,
    pub cover: Option<String>,
    pub meldeschluessel: String,
    pub beschreibung: Option<String>,
    pub link: String,
    pub isbn: Option<String>
}

impl Merkzettel {
    pub fn render(self, templates: &Templates) -> Result<String> {
        Ok(templates.render("merkzettel", &self)?)
    }
}