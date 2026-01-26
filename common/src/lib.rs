use longitude::{Distance, Location};
use thiserror::Error;


#[tarpc::service]
pub trait TripMaker {
    async fn get_random_location(location: Location, max_distance: Distance, number_to_generate: usize) -> Result<Vec<LocationInfo>, LocationError>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LocationInfo {

}

#[derive(Debug, Error, serde::Serialize, serde::Deserialize)]
pub enum LocationError {

}