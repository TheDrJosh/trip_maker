use std::collections::HashMap;

use url::Url;

pub mod nearby_search;
pub mod photos;
pub mod reviews;

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
            .referer(true)
            .build()
            .unwrap();

        Self { client, api_key }
    }

    pub async fn details(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn photos(
        &self,
        location_id: i32,
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
        location_id: i32,
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
        // let mut url = Url::parse("https://api.content.tripadvisor.com/api/v1/location/nearby_search")?;
        let mut url = Url::parse("httpbin.org/")?;
        url.set_query(Some(&serde_url_params::to_string(&WithApiKey {
            key: self.api_key.clone(),
            data: params,
        })?));

        let res_text = self.client.get(url).send().await?.text().await?;

        tracing::info!("{}", res_text);

        Ok(serde_json::from_str(&res_text)?)

        // Ok(self.client.get(url).send().await?.json().await?)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct WithApiKey<T> {
    key: String,
    #[serde(flatten)]
    data: T,
}

#[derive(Debug, serde::Deserialize)]
pub struct Error {
    message: String,
    #[serde(rename = "type")]
    err_type: String,
    code: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Paging {
    next: String,
    previous: String,
    results: i32,
    total_results: i32,
    skipped: i32,
}

#[derive(Debug, serde::Deserialize)]
pub struct User {
    username: String,
    user_location: UserLocation,
    review_count: i32,
    reviewer_badge: String,
    avatar: HashMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserLocation {
    id: String,
    name: String,
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
