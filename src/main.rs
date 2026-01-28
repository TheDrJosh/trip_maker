use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{Router, routing};
use axum_htmx::HxRequestGuardLayer;
use clap::Parser;
use dotenvy::dotenv;

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
        .route("/", routing::get(root));
        // .layer(HxRequestGuardLayer::default());

    axum::serve(listener, app).await.expect("Server Crashed");
}

async fn root() -> &'static str {
    tracing::info!("Hello, World!");
    "Hello, World!"
}
