use crate::trip_advisor::{Address, Error, Language};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

//https://en.wikipedia.org/wiki/ISO_4217

// #[derive(Debug, serde::Serialize)]
// pub enum Currency {
//     #[serde(rename = "AED")]
//     UnitedArabEmiratesDirham,
//     #[serde(rename = "AFN")]
//     AfghanAfghani,
//     #[serde(rename = "ALL")]
//     AlbanianLek,
//     #[serde(rename = "AMD")]
//     ArmenianDram,
// }

#[derive(Debug, serde::Deserialize)]
pub enum Response {
    Error(Error),
    #[serde(untagged)]
    Data(Box<Details>),
}

impl Response {
    pub fn into_result(self) -> Result<Details, Error> {
        match self {
            Response::Data(data) => Ok(*data),
            Response::Error(err) => Err(err),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Details {
    pub location_id: String,
    pub name: String,
    pub description: Option<String>,
    pub web_url: Option<String>,
    pub address_obj: Address,
    pub ancestors: Vec<Ancestor>,
    pub latitude: String,
    pub longitude: String,
    pub timezone: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub write_review: Option<String>,
    pub ranking_data: Option<RankingData>,
    pub rating: Option<String>,
    pub rating_image_url: Option<String>,
    pub num_review: Option<String>,
    pub review_rating_count: Option<ReviewRatingCount>,
    pub photo_count: String,
    pub see_all_photos: String,
    pub price_level: Option<String>,
    pub hours: Option<Hours>,
    pub amenities: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
    pub cuisine: Option<Vec<Name>>,
    pub parent_brand: Option<String>,
    pub brand: Option<String>,
    pub category: Option<Name>,
    pub subcategory: Vec<Name>,
    pub group: Option<Vec<Group>>,
    pub styles: Option<Vec<String>>,
    pub neighborhood_info: Vec<Name>,
    pub trip_types: Vec<TripType>,
    pub awards: Vec<Award>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Ancestor {
    pub abbrv: Option<String>,
    pub level: String,
    pub name: String,
    pub location_id: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct RankingData {
    pub geo_location_id: String,
    pub ranking_string: String,
    pub geo_location_name: String,
    pub ranking_out_of: String,
    pub ranking: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct ReviewRatingCount {
    #[serde(rename = "1")]
    pub one: String,
    #[serde(rename = "2")]
    pub two: String,
    #[serde(rename = "3")]
    pub three: String,
    #[serde(rename = "4")]
    pub four: String,
    #[serde(rename = "5")]
    pub five: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Hours {
    pub periods: Vec<Period>,
    pub weekday_text: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Period {
    pub open: DayTime,
    pub close: DayTime,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct DayTime {
    pub day: i32,
    pub time: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Name {
    pub name: String,
    pub localized_name: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Group {
    #[serde(flatten)]
    pub name: Name,
    pub categories: Vec<Name>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct TripType {
    #[serde(flatten)]
    pub name: Name,
    pub value: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Award {
    pub award_type: String,
    pub year: String,
    pub images: AwardImage,
    pub categories: Vec<String>,
    pub display_name: String,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct AwardImage {
    tiny: String,
    small: String,
    large: String,
}
