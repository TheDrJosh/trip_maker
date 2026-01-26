use longitude::{Distance, Location};


#[tarpc::service]
pub trait TripMaker {
    async fn get_random_location(location: Location, max_distance: Distance, number_to_generate: usize) -> Result<Vec<LocationInfo>, String>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LocationInfo {

}

