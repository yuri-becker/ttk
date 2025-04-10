use crate::api::artikelnummer::Artikelnummer;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub const URL: &str = "https://www.thalia.de/api/rest/public/2003/merkzettel/admin/artikel/v2/";
pub const METHOD: Method = Method::GET;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub artikelnummern: Vec<Artikelnummer>,
}