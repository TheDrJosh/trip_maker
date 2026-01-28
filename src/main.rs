use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    Router,
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue},
    response::{Html, Response},
    routing,
};
use clap::Parser;
use dotenvy::dotenv;
use maud::{PreEscaped, html};
use reqwest::header::CONTENT_TYPE;

mod random_location;
mod trip_advisor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, env, default_value = "false")]
    host: bool,
    #[arg(long, env, default_value = "3000")]
    port: u16,

    #[arg(long, env)]
    api_key: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    _ = dotenv();
    let args = Args::parse();

    let addr = SocketAddr::new(
        if args.host {
            IpAddr::V4(Ipv4Addr::UNSPECIFIED)
        } else {
            IpAddr::V4(Ipv4Addr::LOCALHOST)
        },
        args.port,
    );

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect(&format!("unable to listen on {}", addr));

    tracing::info!("Listening on {}", addr);

    let app = Router::new()
        .route("/", routing::get(root))
        .route("/styles.css", routing::get(styles));

    axum::serve(listener, app).await.expect("Server Crashed");
}

#[axum::debug_handler]
async fn root() -> Html<PreEscaped<String>> {
    Html(layout(html!(
        div class="w-full m-5 border-b border-zinc-500" {
            h1 class="text-5xl text-amber-500 tracking-tight font-bold" {
                "Trip Maker"
            }
        }

    )))
}

#[axum::debug_handler]
async fn styles() -> ([(HeaderName, HeaderValue); 1], &'static str) {
    (
        [(CONTENT_TYPE, HeaderValue::from_static("text/css"))],
        include_str!("../public/output.css"),
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
                title {
                    "Trip Maker"
                }
            }
            body class="bg-zinc-600" {
                (children)
            }
        }

    }
}
