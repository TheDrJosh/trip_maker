use std::sync::Arc;

use common::{LocationInfo, TripMakerClient};
use longitude::{Distance, Location};
use tokio::{sync::Mutex, task::JoinHandle};

#[derive(Default)]
pub struct Connection {
    pub client: Arc<Mutex<Option<common::TripMakerClient>>>,
    pub error: Arc<Mutex<Option<String>>>,
    current_load: Option<JoinHandle<()>>,
}

impl Connection {
    pub fn connect(&mut self, addr: String) {
        if let Some(current_load) = &self.current_load {
            current_load.abort();
        }

        let self_client = self.client.clone();
        let self_error = self.error.clone();

        self.current_load = Some(tokio::spawn(async move {
            match Self::async_connect(addr).await {
                Ok(client) => {
                    *self_error.lock().await = None;
                    *self_client.lock().await = Some(client);
                }
                Err(err) => *self_error.lock().await = Some(err),
            }
        }));
    }

    async fn async_connect(addr: String) -> Result<common::TripMakerClient, String> {
        let addrs = tokio::net::lookup_host(addr)
            .await
            .map_err(|err| err.to_string())?;

        let mut last_err = None;

        for addr in addrs {
            match tarpc::serde_transport::tcp::connect(addr, tokio_serde::formats::Cbor::default)
                .await
            {
                Ok(transport) => {
                    let client =
                        TripMakerClient::new(tarpc::client::Config::default(), transport).spawn();

                    return Ok(client);
                }
                Err(err) => last_err = Some(err.to_string()),
            }
        }

        Err(last_err.unwrap_or("No Connection Available".to_owned()))
    }

    pub fn loading(&self) -> bool {
        self.current_load
            .as_ref()
            .map(|load| !load.is_finished())
            .unwrap_or_default()
    }

    pub fn get_random_location(
        &self,
        location: Location,
        max_distance: Distance,
        number_to_generate: usize,
    ) -> Result<Vec<LocationInfo>, String> {
        // self.locations_error = Some("Server Connection Not Found".to_owned());

        let (res_send, res_receive) = tokio::sync::oneshot::channel();

        let client = self.client.clone();

        tokio::spawn(async move {
            res_send
                .send(
                    Self::get_random_location_async(
                        client,
                        location,
                        max_distance,
                        number_to_generate,
                    )
                    .await,
                )
                .expect("unable to send results to ui thread")
        });

        res_receive.blocking_recv().map_err(|err| err.to_string())?
    }

    async fn get_random_location_async(
        client: Arc<Mutex<Option<common::TripMakerClient>>>,
        location: Location,
        max_distance: Distance,
        number_to_generate: usize,
    ) -> Result<Vec<LocationInfo>, String> {
        if let Some(client) = client.lock().await.as_ref() {
            Ok(client
                .get_random_location(
                    tarpc::context::current(),
                    location,
                    max_distance,
                    number_to_generate,
                    0.0,
                    2.0,
                )
                .await
                .map_err(|err| err.to_string())?
                .map_err(|err| err.to_string())?)
        } else {
            Err("Server Connection Not Found".to_owned())
        }
    }
}
