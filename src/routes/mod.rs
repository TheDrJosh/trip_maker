use axum::{
    Router,
    http::{HeaderName, HeaderValue},
    routing,
};
use maud::html;
use reqwest::header::CONTENT_TYPE;

use crate::state::State;

mod root;

pub fn routes() -> Router<State> {
    Router::new()
        .merge(root::routes())
        .route("/styles.css", routing::get(styles))
}

#[axum::debug_handler]
async fn styles() -> ([(HeaderName, HeaderValue); 1], &'static str) {
    (
        [(CONTENT_TYPE, HeaderValue::from_static("text/css"))],
        include_str!("../../public/output.css"),
    )
}

fn layout(children: maud::PreEscaped<String>) -> maud::PreEscaped<String> {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link href="/styles.css" rel="stylesheet";
                script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js"{}
                script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"{}
                title {
                    "Trip Maker"
                }
            }
            body class="bg-zinc-600 text-zinc-100" {
                (children)
            }
        }

    }
}
