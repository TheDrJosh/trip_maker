use crate::trip_advisor::{Error, Language};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub lat_long: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius_unit: Option<RadiusUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
}

#[derive(Debug, serde::Serialize)]
pub enum RadiusUnit {
    #[serde(rename = "km")]
    Kilometers,
    #[serde(rename = "mi")]
    Miles,
    #[serde(rename = "m")]
    Meters,
}

#[derive(Debug, serde::Deserialize)]
pub enum Response {
    #[serde(rename="data")]
    Data(Option<Vec<Location>>),
    #[serde(rename="error")]
    Error(Option<Error>),
}

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    pub location_id: i32,
    pub name: String,
    pub distance: String,
    pub bearing: String,
    pub address_obj: Address,
}

#[derive(Debug, serde::Deserialize)]
pub struct Address {
    pub street1: String,
    pub street2: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postalcode: String,
    pub address_string: String,
}
