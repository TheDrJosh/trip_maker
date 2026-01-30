use core::f64;
use std::{ops::Bound, sync::Arc};

use axum::{
    Form,
    extract::{State, rejection::FormRejection},
    response::IntoResponse,
};
use longitude::{Distance, DistanceUnit};
use maud::{Markup, html};

use crate::{random_location, state::ServerState};

#[axum::debug_handler]
pub async fn submit(
    State(state): State<Arc<ServerState>>,
    settings: Result<Form<SettingsForm>, FormRejection>,
) -> Result<Markup, ResultError> {
    let settings = settings?.0;

    if !(0.0..).contains(&settings.max_distance) {
        return Err(ResultError::OutOfRangeFloat(
            "max_distance".into(),
            Bound::Included(0.0),
            Bound::Unbounded,
        ));
    }
    if !(0.2..5.0).contains(&settings.closeness_bias) {
        return Err(ResultError::OutOfRangeFloat(
            "closeness_bias".into(),
            Bound::Included(0.2),
            Bound::Included(5.0),
        ));
    }
    if !(0.0..5.0).contains(&settings.minimum_rating) {
        return Err(ResultError::OutOfRangeFloat(
            "minimum_rating".into(),
            Bound::Included(0.0),
            Bound::Included(5.0),
        ));
    }
    if !(0..15).contains(&settings.number_to_generate) {
        return Err(ResultError::OutOfRangeInt(
            "number_to_generate".into(),
            Bound::Included(0),
            Bound::Included(15),
        ));
    }

    let locations = random_location::get_random_location(
        state.client.clone(),
        longitude::Location {
            latitude: settings.latitude,
            longitude: settings.longitude,
        },
        Distance::from(settings.max_distance, settings.distance_unit),
        settings.number_to_generate,
        settings.minimum_rating,
        settings.closeness_bias,
    )
    .await?;

    Ok(html! {
        @for location in &locations {
            ( location_card(location) )
        }
    })
}

fn location_card(location: &random_location::LocationInfo) -> Markup {
    html! {
        div class="border border-zinc-500 rounded-xl p-4 flex flex-col gap-2 min-w-48" {
            h2 class="text-2xl font-bold" { ( &location.name ) }
            @if let Some(description) = &location.description {
                p { ( description ) }
            }
            p {
                "Rating: " ( location.rating )
                " | Distance: " ( location.distance.kilometers() ) " km"
            }
            @if let Some(website) = &location.website {
                a href=(website) class="text-blue-500 underline" { ( website ) }
            }
            p { ( &location.address ) }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResultError {
    #[error("Form rejection: {0}")]
    FormRejection(#[from] FormRejection),
    #[error("Out of Range. {0:?}..{0:?}")]
    OutOfRangeFloat(String, Bound<f64>, Bound<f64>),
    #[error("Out of Range. {0:?}..{0:?}")]
    OutOfRangeInt(String, Bound<usize>, Bound<usize>),
    #[error("Get Random Location error: {0}")]
    GetRandomLocationError(#[from] random_location::GetRandomLocationError),
}

impl IntoResponse for ResultError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            ResultError::OutOfRangeFloat(field, min_bound, max_bound) => html! {
                span class="text-red-500" {
                    "Out of Range. Field: " (field) " Min: " (match min_bound {
                        Bound::Included(bound) => html!{ (bound) },
                        Bound::Excluded(bound) => html!{ "Exclusive: " (bound) },
                        Bound::Unbounded => html!{ "Unbounded" },
                    }) " to Max: " (match max_bound {
                        Bound::Included(bound) => html!{ "Max: " (bound) },
                        Bound::Excluded(bound) => html!{ "Max (exclusive): " (bound) },
                        Bound::Unbounded => html!{ "Max: Unbounded" },
                    })
                }
            },
            ResultError::OutOfRangeInt(field, min_bound, max_bound) => html! {
                span class="text-red-500" {
                    "Out of Range Int. Field: " (field) " Min: " (match min_bound {
                        Bound::Included(bound) => html!{ (bound) },
                        Bound::Excluded(bound) => html!{ "Exclusive: " (bound) },
                        Bound::Unbounded => html!{ "Unbounded" },
                    }) " to " (match max_bound {
                        Bound::Included(bound) => html!{ "Max: " (bound) },
                        Bound::Excluded(bound) => html!{ "Max (exclusive): " (bound) },
                        Bound::Unbounded => html!{ "Max: Unbounded" },
                    })
                }
            },
            ResultError::FormRejection(form_rejection) => html! {
                span class="text-red-500" {
                    "Form Rejection: " (form_rejection)
                }
            },
            ResultError::GetRandomLocationError(get_random_location_error) => html! {
                span class="text-red-500" {
                    "Get Random Location Error: " (get_random_location_error)
                }
            },
        }
        .into_response()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SettingsForm {
    pub longitude: f64,
    pub latitude: f64,
    pub distance_unit: DistanceUnit,
    pub max_distance: f64,
    pub closeness_bias: f64,
    pub minimum_rating: f64,
    pub number_to_generate: usize,
}
