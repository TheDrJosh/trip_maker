use crate::trip_advisor::{Error, Language};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    lat_ong: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius_unit: Option<RadiusUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
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
pub struct Response {
    #[serde(default)]
    data: Option<Vec<Location>>,

    #[serde(default)]
    error: Option<Error>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    location_id: i32,
    name: String,
    distance: String,
    bearing: String,
    address_obj: Address
}

#[derive(Debug, serde::Deserialize)]
pub struct Address {
    street1: String,
    street2: String,
    city: String,
    state: String,
    country: String,
    postalcode: String,
    address_string: String,
}