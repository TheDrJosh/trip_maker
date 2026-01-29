use axum::{Router, routing};
use axum_htmx::HxRequestGuardLayer;

use crate::state::State;

pub mod settings_form;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/settings-form", routing::get(settings_form::page))
        .layer(HxRequestGuardLayer::default())
}
