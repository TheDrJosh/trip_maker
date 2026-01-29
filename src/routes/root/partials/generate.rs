use core::f64;

use axum::Form;
use longitude::DistanceUnit;
use maud::{Markup, html};

#[axum::debug_handler]
pub async fn submit(Form(settings): Form<SettingsForm>) -> Markup {
    html! {
        div id="generated" class="border border-zinc-500 rounded-xl m-2 p-2" {
            (format!("{:?}", settings))
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SettingsForm {
    longitude: f64,
    latitude: f64,
    distance_unit: DistanceUnit,
    max_distance: f64,
    closeness_bias: f64,
    minimum_rating: f64,
    number_to_generate: usize,
}
