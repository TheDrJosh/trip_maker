use core::str;
use std::sync::Arc;

use axum::{
    Router,
    http::{HeaderName, HeaderValue},
    routing,
};
use maud::html;
use reqwest::header::CONTENT_TYPE;

use crate::state::ServerState;

pub mod root;

macro_rules! static_file {
    ($content_type:expr, $data:expr) => {
        || async {
            (
                [(CONTENT_TYPE, HeaderValue::from_static($content_type))],
                $data,
            )
        }
    };
}

pub fn routes() -> Router<Arc<ServerState>> {
    Router::new()
        .merge(root::routes())
        .route(
            "/styles.css",
            routing::get(static_file!(
                "text/css",
                include_bytes!("../../public/output.css")
            )),
        )
        .route(
            "/tail-spin.svg",
            routing::get(static_file!(
                "image/svg+xml",
                include_bytes!("../../public/tail-spin.svg")
            )),
        )
        .route(
            "/htmx.min.js",
            routing::get(static_file!(
                "application/javascript",
                include_bytes!("../../public/htmx.min.js")
            )),
        )
        .route(
            "/alpine.min.js",
            routing::get(static_file!(
                "application/javascript",
                include_bytes!("../../public/alpine.min.js")
            )),
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
                script src="/htmx.min.js"{}
                script defer src="/alpine.min.js"{}
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
