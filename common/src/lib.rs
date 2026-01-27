use longitude::{Distance, Location};

#[tarpc::service]
pub trait TripMaker {
    async fn get_random_location(
        center: Location,
        max_distance: Distance,
        number_to_generate: usize,
        min_rating: f32,
        closeness: f64,
    ) -> Result<Vec<LocationInfo>, String>;
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LocationInfo {}
