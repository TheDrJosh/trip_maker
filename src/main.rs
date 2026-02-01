use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};

use clap::Parser;
use dotenvy::dotenv;
use tower_livereload::LiveReloadLayer;

use crate::state::ServerState;

mod random_location;
mod routes;
mod state;
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
        .unwrap_or_else(|_| panic!("unable to listen on {}", addr));

    tracing::info!("Listening on {}", addr);

    let app = routes::routes().with_state(Arc::new(ServerState {
        client: trip_advisor::TripAdvisor::new(args.api_key),
    }));

    #[cfg(debug_assertions)]
    let app = app.layer(LiveReloadLayer::new());

    axum::serve(listener, app).await.expect("Server Crashed");
}
