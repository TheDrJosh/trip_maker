use std::{cell::RefCell, rc::Rc};

use eframe::egui::{self};
use longitude::{Distance, DistanceUnit, Location};
use url::form_urlencoded::Target;

use crate::{location::LocationOption, trip_advisor::TripAdvisor, utils};

pub struct App {
    current_location: Location,
    settings: Settings,

    open_location: Option<usize>,
    locations: Option<Vec<LocationOption>>,

    client: TripAdvisor,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    api_key: Rc<RefCell<String>>,
    max_distance: f64,
    distance_unit: DistanceUnit,
    number_to_generate: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: Default::default(),
            max_distance: 30.0,
            distance_unit: DistanceUnit::Miles,
            number_to_generate: 5,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Trip Maker");

            ui.separator();

            if let Some(open_location) = self.open_location {
                if let Some(options) = &self.locations {
                    if options[open_location].expanded(ui) {
                        self.open_location = None;
                    }
                } else {
                    self.open_location = None;
                }
            } else {
                self.base_group(ui);
            }
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.settings);
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let pos = public_ip_address::perform_lookup(None).ok();

        let location = Location {
            latitude: pos
                .as_ref()
                .and_then(|pos| pos.latitude)
                .unwrap_or_default(),
            longitude: pos
                .as_ref()
                .and_then(|pos| pos.longitude)
                .unwrap_or_default(),
        };

        let settings: Settings = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        Self {
            current_location: location,
            open_location: None,
            locations: None,
            client: TripAdvisor::new(settings.api_key.clone()),
            settings,
        }
    }

    fn base_group(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Location");
                    ui.label("Latitude");
                    let mut lat_text = self.current_location.latitude.to_string();
                    ui.text_edit_singleline(&mut lat_text);
                    if let Some(new_lat) = lat_text.parse().ok() {
                        self.current_location.latitude = new_lat;
                    } else if lat_text.is_empty() {
                        self.current_location.latitude = 0.0;
                    }

                    ui.label("Longitude");
                    let mut long_text = self.current_location.longitude.to_string();
                    ui.text_edit_singleline(&mut long_text);
                    if let Some(new_long) = long_text.parse().ok() {
                        self.current_location.longitude = new_long;
                    } else if long_text.is_empty() {
                        self.current_location.longitude = 0.0;
                    }

                    if ui.button("Use Current Location").clicked() {
                        let pos = public_ip_address::perform_lookup(None).ok();

                        self.current_location = Location {
                            latitude: pos
                                .as_ref()
                                .and_then(|pos| pos.latitude)
                                .unwrap_or_default(),
                            longitude: pos
                                .as_ref()
                                .and_then(|pos| pos.longitude)
                                .unwrap_or_default(),
                        };
                    }
                });
            });

            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Settings");

                    ui.label("Api Key");
                    ui.text_edit_singleline(self.settings.api_key.borrow_mut().as_mut_string());

                    ui.label("Max Distance*").on_hover_text("This is not exact");

                    ui.horizontal(|ui| {
                        let mut max_distance_str = self.settings.max_distance.to_string();
                        ui.text_edit_singleline(&mut max_distance_str);
                        if let Some(max_distance) = max_distance_str.parse().ok() {
                            self.settings.max_distance = max_distance;
                        } else if max_distance_str.is_empty() {
                            self.settings.max_distance = 0.0;
                        }

                        egui::containers::ComboBox::from_label("")
                            .selected_text(format!("{:?}", self.settings.distance_unit))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Centimeters,
                                    "Centimeter",
                                );
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Feet,
                                    "Feet",
                                );
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Inches,
                                    "Inches",
                                );
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Kilometers,
                                    "Kilometers",
                                );
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Meters,
                                    "Meters",
                                );
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Miles,
                                    "Miles",
                                );
                                ui.selectable_value(
                                    &mut self.settings.distance_unit,
                                    DistanceUnit::Yards,
                                    "Yards",
                                );
                            })
                    });

                    ui.label("Number to Generate");
                    let mut number_to_generate_str = self.settings.number_to_generate.to_string();

                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut number_to_generate_str);
                        if let Some(number_to_generate) = number_to_generate_str.parse().ok() {
                            self.settings.number_to_generate = number_to_generate;
                        } else if number_to_generate_str.is_empty() {
                            self.settings.number_to_generate = 0;
                        }
                        if ui.button("+").clicked() {
                            self.settings.number_to_generate += 1;
                        }
                        if ui.button("-").clicked() && self.settings.number_to_generate != 0 {
                            self.settings.number_to_generate -= 1;
                        }
                    });
                });
            });
        });

        if ui.button("Generate").clicked() {
            self.generate_options();
        }

        if let Some(locations) = &self.locations {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    for (i, location) in locations.iter().enumerate() {
                        if location.preview(ui) {
                            self.open_location = Some(i);
                        }
                    }
                });
            });
        }
    }

    fn generate_options(&mut self) {
        let mut locations = Vec::with_capacity(self.settings.number_to_generate as usize);

        while locations.len() < self.settings.number_to_generate as usize {
            let point = utils::get_rand_cord(
                &self.current_location,
                &Distance::from(self.settings.max_distance, self.settings.distance_unit),
            );
            println!("{:?}", point);
            locations.push(LocationOption {});
        }

        self.locations = Some(locations);
    }
}
