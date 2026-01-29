use core::f64;
use std::{
    num::{ParseFloatError, ParseIntError},
    ops::{Bound, Deref},
};

use axum::{Form, response::Html};
use longitude::DistanceUnit;
use maud::{Markup, PreEscaped, html};
use serde::Deserialize;

#[axum::debug_handler]
pub async fn page(Form(settings): Form<SettingsForm>) -> Html<PreEscaped<String>> {
    Html(settings_form(Some(settings)))
}

#[derive(Deserialize)]
pub struct SettingsForm {
    longitude: String,
    latitude: String,
    distance_unit: String,
    max_distance: String,
    closeness_bias: String,
    minimum_rating: String,
    number_to_generate: String,
}

impl SettingsForm {
    pub fn validate(&self) -> SettingsFormValidation {
        let longitude = self
            .longitude
            .parse::<f64>()
            .map_err(SettingsFormValidationError::from);

        let latitude = self
            .latitude
            .parse::<f64>()
            .map_err(SettingsFormValidationError::from);

        let distance_unit = match self.distance_unit.deref() {
            "Centimeters" => DistanceUnit::Centimeters,
            "Meters" => DistanceUnit::Meters,
            "Kilometers" => DistanceUnit::Kilometers,
            "Inches" => DistanceUnit::Inches,
            "Feet" => DistanceUnit::Feet,
            "Yards" => DistanceUnit::Yards,
            "Miles" => DistanceUnit::Miles,
            _ => DistanceUnit::Miles,
        };

        let max_distance = self
            .max_distance
            .parse::<f64>()
            .map_err(SettingsFormValidationError::from)
            .and_then(|val| {
                if val >= 0.0 {
                    Ok(val)
                } else {
                    Err(SettingsFormValidationError::OutOfRange(
                        Bound::Included(0.0),
                        Bound::Unbounded,
                    ))
                }
            });

        let closeness_bias = self
            .closeness_bias
            .parse::<f64>()
            .map_err(SettingsFormValidationError::from)
            .and_then(|val| {
                if (0.2..=5.0).contains(&val) {
                    Ok(val)
                } else {
                    Err(SettingsFormValidationError::OutOfRange(
                        Bound::Included(0.2),
                        Bound::Included(5.0),
                    ))
                }
            });

        let minimum_rating = self
            .minimum_rating
            .parse::<f64>()
            .map_err(SettingsFormValidationError::from)
            .and_then(|val| {
                if (0.0..=5.0).contains(&val) {
                    Ok(val)
                } else {
                    Err(SettingsFormValidationError::OutOfRange(
                        Bound::Included(0.0),
                        Bound::Included(5.0),
                    ))
                }
            });

        let number_to_generate = self
            .number_to_generate
            .parse::<usize>()
            .map_err(SettingsFormValidationError::from)
            .and_then(|val| {
                if (0..16).contains(&val) {
                    Ok(val)
                } else {
                    Err(SettingsFormValidationError::OutOfRange(
                        Bound::Included(0.0),
                        Bound::Included(16.0),
                    ))
                }
            });

        SettingsFormValidation {
            longitude,
            latitude,
            distance_unit,
            max_distance,
            closeness_bias,
            minimum_rating,
            number_to_generate,
        }
    }
}

pub struct SettingsFormValidation {
    longitude: Result<f64, SettingsFormValidationError>,
    latitude: Result<f64, SettingsFormValidationError>,
    distance_unit: DistanceUnit,
    max_distance: Result<f64, SettingsFormValidationError>,
    closeness_bias: Result<f64, SettingsFormValidationError>,
    minimum_rating: Result<f64, SettingsFormValidationError>,
    number_to_generate: Result<usize, SettingsFormValidationError>,
}

impl SettingsFormValidation {
    // pub fn into_info(self) -> Result<SettingsFormInfo, SettingsFormValidationError> {
    //     Ok(SettingsFormInfo {
    //         longitude: self.longitude?,
    //         latitude: self.latitude?,
    //         distance_unit: self.distance_unit,
    //         max_distance: self.max_distance?,
    //         closeness_bias: self.closeness_bias?,
    //         minimum_rating: self.minimum_rating?,
    //         number_to_generate: self.number_to_generate?,
    //     })
    // }

    pub fn has_error(&self) -> bool {
        self.longitude.is_err()
            || self.latitude.is_err()
            || self.max_distance.is_err()
            || self.closeness_bias.is_err()
            || self.minimum_rating.is_err()
            || self.number_to_generate.is_err()
    }
}

pub struct SettingsFormInfo {
    longitude: f64,
    latitude: f64,
    distance_unit: DistanceUnit,
    max_distance: f64,
    closeness_bias: f64,
    minimum_rating: f64,
    number_to_generate: usize,
}

impl Default for SettingsFormInfo {
    fn default() -> Self {
        Self {
            longitude: 0.0,
            latitude: 0.0,
            distance_unit: DistanceUnit::Miles,
            max_distance: 10.0,
            closeness_bias: 1.0,
            minimum_rating: 0.0,
            number_to_generate: 5,
        }
    }
}

impl SettingsFormInfo {
    pub fn to_strings(&self) -> SettingsForm {
        SettingsForm {
            longitude: self.longitude.to_string(),
            latitude: self.latitude.to_string(),
            distance_unit: match &self.distance_unit {
                DistanceUnit::Centimeters => "Centimeters".to_owned(),
                DistanceUnit::Meters => "Meters".to_owned(),
                DistanceUnit::Kilometers => "Kilometers".to_owned(),
                DistanceUnit::Inches => "Inches".to_owned(),
                DistanceUnit::Feet => "Feet".to_owned(),
                DistanceUnit::Yards => "Yards".to_owned(),
                DistanceUnit::Miles => "Miles".to_owned(),
            },
            max_distance: self.max_distance.to_string(),
            closeness_bias: self.closeness_bias.to_string(),
            minimum_rating: self.minimum_rating.to_string(),
            number_to_generate: self.number_to_generate.to_string(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SettingsFormValidationError {
    #[error("{0}")]
    ParseErrorFloat(#[from] ParseFloatError),
    #[error("{0}")]
    ParseErrorInt(#[from] ParseIntError),
    #[error("Out of Range. {0:?}..{1:?}")]
    OutOfRange(Bound<f64>, Bound<f64>),
}

pub fn settings_form(settings_form: Option<SettingsForm>) -> maud::Markup {
    let settings_form = settings_form.unwrap_or(SettingsFormInfo::default().to_strings());
    let settings_validations = settings_form.validate();

    html! {
        form class="flex flex-col gap-4" id="settings-form" hx-get="/partial/settings-form" hx-trigger="submit, change" {
            div class="border border-zinc-500 rounded-xl px-2 py-1" {
                div class="flex flex-row flex-wrap gap-4" {
                    (center_settings(&settings_form, &settings_validations))
                    (distance_settings(&settings_form, &settings_validations))
                    (other_settings(&settings_form, &settings_validations))
                }
            }
            button type="submit" class="bg-zinc-700 px-2 py-1 rounded-lg text-lg font-bold tracking-tight self-start hover:bg-zinc-800 hover:text-amber-500" disabled[settings_validations.has_error()] {
                "Generate"
            }
        }
    }
}

fn center_settings(settings_form: &SettingsForm, validations: &SettingsFormValidation) -> Markup {
    html! {
        div class="flex flex-col gap-2 flex-1" {
            div class="flex flex-row gap-2 items-center" {
                label for="longitude" {
                    "Longitude"
                }
                input name="longitude" id="longitude" type="number" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value=(settings_form.longitude);
            }
            (error_text(&validations.longitude))
            div class="flex flex-row gap-2 items-center" {
                label for="latitude" {
                    "Latitude"
                }
                input name="latitude" id="latitude" type="number" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value=(settings_form.latitude);
            }
            (error_text(&validations.latitude))
            button type="button" class="bg-zinc-700 px-2 py-1 rounded-lg hover:bg-zinc-800 self-start" onclick="navigator.geolocation.getCurrentPosition((data) => {document.getElementById(\"test\").innerHTML = data.coords.longitude + \", \" + data.coords.latitude;}, (err) => {document.getElementById(\"test\").innerHTML = err.code + \" | \" + err.message}, {enableHighAccuracy: true});" {
                "Current Position"
            }
            p id="test" {}
            button class="bg-zinc-700 px-2 py-1 rounded-lg hover:bg-zinc-800 self-start" {
                "Set Address"
            }
        }
    }
}

fn distance_settings(settings_form: &SettingsForm, validations: &SettingsFormValidation) -> Markup {
    html! {
        div class="flex flex-col gap-2 flex-1" {
            div class="flex flex-row gap-2 items-center" {
                label for="distance_unit" {
                    "Distance Unit"
                }
                select name="distance_unit" class="bg-zinc-700 px-1 py-0.5 rounded-lg" {
                    option value="Centimeters" selected[validations.distance_unit == DistanceUnit::Centimeters] {
                        "Centimeters"
                    }
                    option value="Meters" selected[validations.distance_unit == DistanceUnit::Meters] {
                        "Meters"
                    }
                    option value="Kilometers" selected[validations.distance_unit == DistanceUnit::Kilometers] {
                        "Kilometers"
                    }
                    option value="Inches" selected[validations.distance_unit == DistanceUnit::Inches] {
                        "Inches"
                    }
                    option value="Feet" selected[validations.distance_unit == DistanceUnit::Feet] {
                        "Feet"
                    }
                    option value="Yards" selected[validations.distance_unit == DistanceUnit::Yards] {
                        "Yards"
                    }
                    option value="Miles" selected[validations.distance_unit == DistanceUnit::Miles] {
                        "Miles"
                    }
                }
            }
            div class="flex flex-row gap-2 items-center" {
                label for="max_distance" {
                    "Max Distance"
                }
                input name="max_distance" type="number" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value=(settings_form.max_distance) min="0";
            }
            (error_text(&validations.max_distance))
        }
    }
}

fn other_settings(settings_form: &SettingsForm, validations: &SettingsFormValidation) -> Markup {
    html! {
        div class="flex flex-col gap-2 flex-1" {
            div class="flex flex-row gap-2 items-center" {
                label for="closeness_bias" {
                    "Closeness Bias"
                }
                input name="closeness_bias" type="number" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value=(settings_form.closeness_bias) min="0.2" max="5" step="0.1";
            }
            (error_text(&validations.closeness_bias))
            div class="flex flex-row gap-2 items-center" {
                label for="minimum_rating" {
                    "Minimum Rating"
                }
                input name="minimum_rating" type="number" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value=(settings_form.minimum_rating) min="0" max="5" step="0.1";
            }
            (error_text(&validations.minimum_rating))
            div class="flex flex-row gap-2 items-center" {
                label for="number_to_generate" {
                    "Number To Generate"
                }
                input name="number_to_generate" type="number" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value=(settings_form.number_to_generate) min="0";
            }
            (error_text(&validations.number_to_generate))
        }
    }
}

fn error_text<T>(res: &Result<T, SettingsFormValidationError>) -> Markup {
    if let Err(err) = res {
        html! {
            p class="text-red-500" {
                (err.to_string())
            }
        }
    } else {
        html! {}
    }
}
