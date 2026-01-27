use crate::trip_advisor::{Address, Error, Language};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub lat_long: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
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
#[serde(rename_all = "lowercase")]
pub enum Category {
    Hotels,
    Attractions,
    Restaurants,
    Geos,
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
#[serde(rename_all="lowercase")]
pub enum Response {
    Data(Vec<Location>),
    // #[serde(untagged)]
    Error(Error),
}



impl Response {
    pub fn to_result(self) -> Result<Vec<Location>, Error> {
        match self {
            Response::Data(data) => Ok(data),
            Response::Error(err) => Err(err),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Location {
    pub location_id: String,
    pub name: String,
    pub distance: String,
    pub bearing: String,
    pub address_obj: Address,
}
