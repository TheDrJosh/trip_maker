use std::collections::HashMap;

use serde_repr::Deserialize_repr;

use crate::trip_advisor::{Error, Language, Paging, User};

#[derive(Debug, serde::Serialize)]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    #[serde(default)]
    data: Option<Vec<Review>>,

    #[serde(default)]
    paging: Option<Paging>,

    #[serde(default)]
    error: Option<Error>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Review {
    id: i32,
    lang: Language,
    location_id: i32,
    published_data: String,
    rating: Rating,
    helpful_votes: i32,
    rating_image_url: String,
    url: String,
    trip_type: TripType,
    travel_date: String,
    text: String,
    title: String,
    owner_response: OwnerResponse,
    is_machine_translated: bool,
    user: User,
    subratings: HashMap<String, Subrating>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OwnerResponse {
    id: i32,
    lang: Language,
    text: String,
    title: String,
    author: String,
    published_date: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Subrating {
    name: String,
    localized_name: String,
    rating_image_url: String,
    value: f32,
}

#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
enum Rating {
    Terrible = 1,
    Poor,
    Average,
    VeryGood,
    Excellent,
}

#[derive(Debug, serde::Deserialize)]
enum TripType {
    Business,
    Couples,
    Family,
    Friends,
    Solo,
}
