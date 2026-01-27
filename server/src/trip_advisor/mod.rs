use std::{collections::HashMap, fmt::Debug};

use url::Url;

pub mod details;
pub mod nearby_search;
pub mod photos;
pub mod reviews;
pub mod search;

#[derive(Clone)]
pub struct TripAdvisor {
    client: reqwest::Client,
    api_key: String,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

impl TripAdvisor {
    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        Self { client, api_key }
    }

    pub async fn details(
        &self,
        location_id: String,
        params: details::Params,
    ) -> anyhow::Result<details::Details> {
        let mut url = Url::parse(&format!(
            "https://api.content.tripadvisor.com/api/v1/location/{}/details",
            location_id
        ))?;
        url.set_query(Some(&serde_url_params::to_string(&WithApiKey {
            key: self.api_key.clone(),
            data: params,
        })?));

        Ok(self
            .client
            .get(url)
            .send()
            .await
            .map_err(log_err)?
            .json()
            .await
            .map_err(log_err)?)
    }

    pub async fn photos(
        &self,
        location_id: String,
        params: photos::Params,
    ) -> anyhow::Result<photos::Response> {
        let mut url = Url::parse(&format!(
            "https://api.content.tripadvisor.com/api/v1/location/{}/photos",
            location_id
        ))?;
        url.set_query(Some(&serde_url_params::to_string(&WithApiKey {
            key: self.api_key.clone(),
            data: params,
        })?));

        Ok(self.client.get(url).send().await?.json().await?)
    }

    pub async fn reviews(
        &self,
        location_id: String,
        params: reviews::Params,
    ) -> anyhow::Result<reviews::Response> {
        let mut url = Url::parse(&format!(
            "https://api.content.tripadvisor.com/api/v1/location/{}/reviews",
            location_id
        ))?;
        url.set_query(Some(&serde_url_params::to_string(&WithApiKey {
            key: self.api_key.clone(),
            data: params,
        })?));

        Ok(self.client.get(url).send().await?.json().await?)
    }

    pub async fn search(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn nearby_search(
        &self,
        params: nearby_search::Params,
    ) -> anyhow::Result<nearby_search::Response> {
        let mut url =
            Url::parse("https://api.content.tripadvisor.com/api/v1/location/nearby_search")?;
        url.set_query(Some(&serde_url_params::to_string(&WithApiKey {
            key: self.api_key.clone(),
            data: params,
        })?));

        let text = self
            .client
            .get(url)
            .send()
            .await
            .map_err(log_err)?
            .text()
            .await
            .map_err(log_err)?;

        // tracing::info!("{}", text);

        Ok(serde_json::from_str(&text).map_err(log_err)?)

        // Ok(self
        //     .client
        //     .get(url)
        //     .send()
        //     .await
        //     .map_err(log_err)?
        //     .json()
        //     .await
        //     .map_err(log_err)?)
    }
}

fn log_err<T: Debug>(err: T) -> T {
    tracing::error!("{:?}", err);
    err
}

#[derive(Debug, serde::Serialize)]
pub struct WithApiKey<T> {
    key: String,
    #[serde(flatten)]
    data: T,
}

#[derive(Debug, serde::Deserialize)]
pub struct Error {
    #[serde(rename = "Message")]
    pub message: String,
    // #[serde(rename = "Type")]
    // pub err_type: String,
    // pub code: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Paging {
    pub next: String,
    pub previous: String,
    pub results: i32,
    pub total_results: i32,
    pub skipped: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub user_location: UserLocation,
    pub review_count: i32,
    pub reviewer_badge: String,
    pub avatar: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserLocation {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Address {
    pub street1: String,
    pub street2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postalcode: String,
    pub address_string: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Language {
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "zh")]
    Chinese,
    #[serde(rename = "zh_TW")]
    ChineseTaiwan,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "en_AU")]
    EnglishAustralia,
    #[serde(rename = "en_CA")]
    EnglishCanada,
    #[serde(rename = "en_HK")]
    EnglishHongKong,
    #[serde(rename = "en_IN")]
    EnglishIndia,
    #[serde(rename = "en_IE")]
    EnglishIreland,
    #[serde(rename = "en_MY")]
    EnglishMalaysia,
    #[serde(rename = "en_NZ")]
    EnglishNewZealand,
    #[serde(rename = "en_PH")]
    EnglishPhilippines,
    #[serde(rename = "en_SG")]
    EnglishSingapore,
    #[serde(rename = "en_ZA")]
    EnglishSouthAfrica,
    #[serde(rename = "en_UK")]
    EnglishUnitedKingdom,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "fr_BE")]
    FrenchBelgium,
    #[serde(rename = "fr_CA")]
    FrenchCanada,
    #[serde(rename = "fr_CH")]
    FrenchSwitzerland,
    #[serde(rename = "de_AT")]
    GermanAustria,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "iw")]
    Hebrew,
    #[serde(rename = "in")]
    Indonesian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "it_CH")]
    ItalianSwitzerland,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "pt_PT")]
    PortuguesePortugal,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "es_AR")]
    SpanishArgentina,
    #[serde(rename = "es_CO")]
    SpanishColombia,
    #[serde(rename = "es_MX")]
    SpanishMexico,
    #[serde(rename = "es_PE")]
    SpanishPeru,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "es_VE")]
    SpanishVenezuela,
    #[serde(rename = "es_CL")]
    SpanishChile,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "vi")]
    Vietnamese,
}
