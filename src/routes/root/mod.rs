use axum::{Router, response::Html, routing};
use maud::{PreEscaped, html};

use crate::{routes::{layout, root::partials::settings_form::settings_form}, state::State};

pub mod partials;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/", routing::get(page))
        .nest("/partial", partials::routes())
}

#[axum::debug_handler]
async fn page() -> Html<PreEscaped<String>> {
    Html(layout(html!(
        div class="w-full p-5 border-b border-zinc-500" {
            h1 class="text-5xl text-amber-500 tracking-tight font-bold" {
                "Trip Maker"
            }
        }
        div class="w-full p-4" {
            (settings_form(None))
        }

    )))
}
