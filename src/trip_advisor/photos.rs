use eframe::egui::ahash::HashMap;

use crate::trip_advisor::{Error, Language, Paging, User};

#[derive(Debug, serde::Serialize)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
}

#[derive(Debug, serde::Serialize)]
pub enum Source {
    Expert,
    Management,
    Traveler,
}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    #[serde(default)]
    data: Option<Vec<ImageSet>>,

    #[serde(default)]
    paging: Option<Paging>,

    #[serde(default)]
    error: Option<Error>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ImageSet {
    id: i32,
    is_blessed: bool,
    album: String,
    caption: String,
    published_data: String,
    images: HashMap<String, ImageProps>,
    source: ImageSource,
    user: User,
}

#[derive(Debug, serde::Deserialize)]
pub struct ImageProps {
    width: u32,
    height: u32,
    url: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ImageSource {
    name: String,
    localized_name: String,
}