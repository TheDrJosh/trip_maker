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
    pub longitude: f64,
    pub latitude: f64,
    pub distance_unit: DistanceUnit,
    pub max_distance: f64,
    pub closeness_bias: f64,
    pub minimum_rating: f64,
    pub number_to_generate: usize,
}
