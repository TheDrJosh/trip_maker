use longitude::{Distance, Location};

#[tarpc::service]
pub trait TripMaker {
    async fn get_random_location(
        center: Location,
        max_distance: Distance,
        number_to_generate: usize,
        min_rating: f64,
        closeness: f64,
    ) -> Result<Vec<LocationInfo>, String>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LocationInfo {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub rating: f64,
    pub address: String,
}
