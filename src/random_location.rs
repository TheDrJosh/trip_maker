use longitude::{Direction, Distance, Location};
use rand::Rng;

use crate::trip_advisor::{self, TripAdvisor};

fn get_rand_cord(center: &Location, radius: &Distance, closeness: f64) -> Location {
    let mut rand = rand::rng();
    let angle = rand.random::<f64>() * std::f64::consts::PI;
    let dist = rand.random::<f64>();

    let dist = dist.powf(closeness);

    let distance_north = Distance::from_kilometers(radius.kilometers() * dist * angle.sin());
    let distance_east = Distance::from_kilometers(radius.kilometers() * dist * angle.cos());

    center
        .add(&distance_north, Direction::North)
        .add(&distance_east, Direction::East)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LocationInfo {
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub rating: f64,
    pub address: String,
    pub distance: Distance,
}

pub async fn get_random_location(
    client: TripAdvisor,
    center: Location,
    max_distance: Distance,
    number_to_generate: usize,
    min_rating: f64,
    closeness: f64,
) -> Result<Vec<LocationInfo>, GetRandomLocationError> {
    let mut locations = Vec::with_capacity(number_to_generate);

    while locations.len() < number_to_generate {
        let rand_loc = get_rand_cord(&center, &max_distance, closeness);

        for location in client
            .nearby_search(trip_advisor::nearby_search::Params {
                lat_long: rand_loc.to_string(),
                category: Some(trip_advisor::nearby_search::Category::Attractions),
                phone: None,
                address: None,
                radius: Some((max_distance.kilometers() * 2.0).to_string()),
                radius_unit: Some(trip_advisor::nearby_search::RadiusUnit::Kilometers),
                language: None,
            })
            .await?
            .into_result()?
        {
            let details = client
                .details(
                    location.location_id,
                    trip_advisor::details::Params {
                        language: None,
                        currency: None,
                    },
                )
                .await?
                .into_result()?;

            let rating = details
                .rating
                .map(|rating| rating.parse::<f64>().unwrap())
                .unwrap_or_default();

            let distance = center.distance(&Location {
                latitude: details.latitude.parse().unwrap(),
                longitude: details.longitude.parse().unwrap(),
            });

            if rating >= min_rating && distance < max_distance {
                locations.push(LocationInfo {
                    name: details.name,
                    description: details.description,
                    website: details.website,
                    rating,
                    address: details.address_obj.address_string,
                    distance,
                });
                break;
            }
        }
    }

    Ok(locations)
}

#[derive(Debug, thiserror::Error)]
pub enum GetRandomLocationError {
    #[error("TripAdvisor API error: {0}")]
    TripAdvisorClientError(#[from] trip_advisor::TripAdvisorError),
    #[error("TripAdvisor API error: {0}")]
    TripAdvisorServerError(#[from] trip_advisor::Error),
}
