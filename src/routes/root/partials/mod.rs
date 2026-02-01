use std::sync::Arc;

use axum::{Router, routing};
use axum_htmx::HxRequestGuardLayer;

use crate::state::ServerState;

pub mod generate;
pub mod settings;

pub fn routes() -> Router<Arc<ServerState>> {
    Router::new()
        .route("/generate", routing::post(generate::submit))
        .route("/check", routing::get(settings::check_setting))
        .layer(HxRequestGuardLayer::default())
}
