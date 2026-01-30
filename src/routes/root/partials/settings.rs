use std::{
    num::{ParseFloatError, ParseIntError},
    ops::{Bound, RangeBounds},
};

use axum::{Form, response::IntoResponse};
use axum_htmx::HxTrigger;
use maud::html;

#[derive(Debug, serde::Deserialize)]
pub struct Setting {
    #[serde(default)]
    pub longitude: Option<String>,
    #[serde(default)]
    pub latitude: Option<String>,
    #[serde(default)]
    pub max_distance: Option<String>,
    #[serde(default)]
    pub closeness_bias: Option<String>,
    #[serde(default)]
    pub minimum_rating: Option<String>,
    #[serde(default)]
    pub number_to_generate: Option<String>,
}

#[axum::debug_handler]
pub async fn check_setting(
    HxTrigger(trigger): HxTrigger,
    Form(setting): Form<Setting>,
) -> Result<(), CheckSettingError> {
    match trigger.as_deref() {
        Some("longitude") => {
            setting
                .longitude
                .ok_or(CheckSettingError::DataNotFound)?
                .parse::<f64>()?;
        }
        Some("latitude") => {
            setting
                .latitude
                .ok_or(CheckSettingError::DataNotFound)?
                .parse::<f64>()?;
        }
        Some("max_distance") => {
            let range = 0.0..;

            if !range.contains(
                &setting
                    .max_distance
                    .ok_or(CheckSettingError::DataNotFound)?
                    .parse::<f64>()?,
            ) {
                Err(CheckSettingError::OutOfRangeFloat(
                    range.start_bound().cloned(),
                    range.end_bound().cloned(),
                ))?;
            }
        }
        Some("closeness_bias") => {
            let range = 0.2..=5.0;

            if !range.contains(
                &setting
                    .closeness_bias
                    .ok_or(CheckSettingError::DataNotFound)?
                    .parse::<f64>()?,
            ) {
                Err(CheckSettingError::OutOfRangeFloat(
                    range.start_bound().cloned(),
                    range.end_bound().cloned(),
                ))?;
            }
        }
        Some("minimum_rating") => {
            let range = 0.0..=5.0;

            if !range.contains(
                &setting
                    .minimum_rating
                    .ok_or(CheckSettingError::DataNotFound)?
                    .parse::<f64>()?,
            ) {
                Err(CheckSettingError::OutOfRangeFloat(
                    range.start_bound().cloned(),
                    range.end_bound().cloned(),
                ))?;
            }
        }
        Some("number_to_generate") => {
            let range = 0..=15;

            if !range.contains(
                &setting
                    .number_to_generate
                    .ok_or(CheckSettingError::DataNotFound)?
                    .parse::<usize>()?,
            ) {
                Err(CheckSettingError::OutOfRangeInt(
                    range.start_bound().cloned(),
                    range.end_bound().cloned(),
                ))?;
            }
        }
        _ => Err(CheckSettingError::UnknownField)?,
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum CheckSettingError {
    #[error("Unknown Field")]
    UnknownField,
    #[error("Data Not Found")]
    DataNotFound,
    #[error("Out of Range. {0:?}..{0:?}")]
    OutOfRangeFloat(Bound<f64>, Bound<f64>),
    #[error("Out of Range. {0:?}..{0:?}")]
    OutOfRangeInt(Bound<usize>, Bound<usize>),
    #[error("{0}")]
    ParseFloat(#[from] ParseFloatError),
    #[error("{0}")]
    ParseInt(#[from] ParseIntError),
}

impl IntoResponse for CheckSettingError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            CheckSettingError::UnknownField => html! {
                "Unknown Field"
            },
            CheckSettingError::DataNotFound => html! {
                "Data Not Found"
            },
            CheckSettingError::OutOfRangeFloat(min_bound, max_bound) => html! {
                "Out of Range. " (match min_bound {
                    Bound::Included(bound) => html!{ "Min: " (bound) },
                    Bound::Excluded(bound) => html!{ "Min (exclusive): " (bound) },
                    Bound::Unbounded => html!{ "Min: Unbounded" },
                }) " to " (match max_bound {
                    Bound::Included(bound) => html!{ "Max: " (bound) },
                    Bound::Excluded(bound) => html!{ "Max (exclusive): " (bound) },
                    Bound::Unbounded => html!{ "Max: Unbounded" },
                })
            },
            CheckSettingError::OutOfRangeInt(min_bound, max_bound) => html! {
                "Out of Range Int. " (match min_bound {
                    Bound::Included(bound) => html!{ "Min: " (bound) },
                    Bound::Excluded(bound) => html!{ "Min (exclusive): " (bound) },
                    Bound::Unbounded => html!{ "Min: Unbounded" },
                }) " to " (match max_bound {
                    Bound::Included(bound) => html!{ "Max: " (bound) },
                    Bound::Excluded(bound) => html!{ "Max (exclusive): " (bound) },
                    Bound::Unbounded => html!{ "Max: Unbounded" },
                })
            },
            CheckSettingError::ParseFloat(parse_float_error) => html! {
                "Parse Float Error: " (parse_float_error)
            },
            CheckSettingError::ParseInt(parse_int_error) => html! {
                "Parse Int Error: " (parse_int_error)
            },
        }
        .into_response()
    }
}
