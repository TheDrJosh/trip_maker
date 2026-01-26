use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;
use common::{LocationError, LocationInfo, TripMaker};
use dotenvy::dotenv;
use futures::StreamExt;
use longitude::{Direction, Distance, Location};
use rand::Rng;
use tarpc::{
    context::Context,
    server::{Channel, incoming::Incoming},
};

use crate::trip_advisor::TripAdvisor;

mod trip_advisor;

#[derive(Clone)]
struct Server {
    addr: SocketAddr,
    client: TripAdvisor,
}

pub fn get_rand_cord(center: &Location, radius: &Distance) -> Location {
    let mut rand = rand::rng();
    let angle = rand.random::<f64>() * std::f64::consts::PI;
    let dist = rand.random::<f64>();

    let distance_north = Distance::from_kilometers(radius.kilometers() * dist * dist * angle.sin());
    let distance_east = Distance::from_kilometers(radius.kilometers() * dist * dist * angle.cos());

    center
        .add(&distance_north, Direction::North)
        .add(&distance_east, Direction::East)
}

impl TripMaker for Server {
    async fn get_random_location(
        self,
        _context: Context,
        location: Location,
        max_distance: Distance,
        number_to_generate: usize,
    ) -> Result<Vec<LocationInfo>, LocationError> {
        let mut locations = Vec::with_capacity(number_to_generate);

        while locations.len() < number_to_generate {
            locations.push(LocationInfo {});
        }

        Ok(locations)
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, env)]
    addr: Option<std::net::IpAddr>,
    #[arg(long, env)]
    port: Option<u16>,

    #[arg(long, env)]
    api_key: String,
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    _ = dotenv();
    let args = Args::parse();

    let addr = SocketAddr::new(
        args.addr.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        args.port.unwrap_or(21581),
    );

    tracing::info!("Listening on {}", addr);

    let listener = tarpc::serde_transport::tcp::listen(addr, tokio_serde::formats::Cbor::default)
        .await
        .unwrap();

    listener
        .filter_map(|r| futures::future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = Server {
                addr: channel.transport().peer_addr().unwrap(),
                client: TripAdvisor::new(args.api_key.clone()),
            };
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;
}
