use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;
use common::{LocationInfo, TripMaker};
use dotenvy::dotenv;
use futures::StreamExt;
use longitude::{Direction, Distance, Location};
use rand::Rng;
use tarpc::{
    context::Context,
    server::{Channel, incoming::Incoming},
};

use crate::trip_advisor::{TripAdvisor, details};

mod trip_advisor;

#[derive(Clone)]
struct Server {
    addr: SocketAddr,
    client: TripAdvisor,
}

pub fn get_rand_cord(center: &Location, radius: &Distance, closeness: f64) -> Location {
    let mut rand = rand::rng();
    let angle = rand.random::<f64>() * std::f64::consts::PI;
    let dist = rand.random::<f64>();

    let dist = dist.powf(closeness);

    let distance_north = Distance::from_kilometers(radius.kilometers() * dist * angle.sin());
    let distance_east = Distance::from_kilometers(radius.kilometers() * dist * angle.cos());

    center
        .add(&distance_north, Direction::North)
        .add(&distance_east, Direction::East)
}

impl TripMaker for Server {
    async fn get_random_location(
        self,
        _context: Context,
        center: Location,
        max_distance: Distance,
        number_to_generate: usize,
        min_rating: f32,
        closeness: f64,
    ) -> Result<Vec<LocationInfo>, String> {
        let mut locations = Vec::with_capacity(number_to_generate);

        while locations.len() < number_to_generate {
            let rand_loc = get_rand_cord(&center, &max_distance, closeness);

            let res = futures::future::join_all(
                self.client
                    .nearby_search(trip_advisor::nearby_search::Params {
                        lat_long: rand_loc.to_string(),
                        category: Some(trip_advisor::nearby_search::Category::Attractions),
                        phone: None,
                        address: None,
                        radius: Some((max_distance.kilometers() * 2.0).to_string()),
                        radius_unit: Some(trip_advisor::nearby_search::RadiusUnit::Kilometers),
                        language: None,
                    })
                    .await
                    .map_err(|err| err.to_string())?
                    .to_result()
                    .map_err(|err| err.message)?
                    .data
                    .into_iter()
                    .map(|loc| {
                        self.client.details(
                            loc.location_id,
                            trip_advisor::details::Params {
                                language: None,
                                currency: None,
                            },
                        )
                    }),
            )
            .await
            .into_iter()
            .filter_map(|res| res.ok())
            .collect::<Vec<details::Details>>();

            println!("{:?}", res);

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
