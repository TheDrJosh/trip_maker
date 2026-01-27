use common::LocationInfo;
use eframe::egui::{self, Color32, RichText};
use longitude::{Distance, DistanceUnit, Location};

use crate::connection::Connection;

pub struct App {
    current_position: Location,
    settings: Settings,

    connection: Connection,

    open_location: Option<usize>,
    locations: Option<Vec<LocationInfo>>,
    locations_error: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    server_addr: String,
    max_distance: f64,
    distance_unit: DistanceUnit,
    number_to_generate: usize,
    closeness_bias: f64,
    minimum_rating: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_addr: Default::default(),
            max_distance: 30.0,
            distance_unit: DistanceUnit::Miles,
            number_to_generate: 5,
            closeness_bias: 1.0,
            minimum_rating: 0.0,
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
                    if location_expanded(ui, &options[open_location]) {
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

        let mut connection = Connection::default();

        connection.connect(settings.server_addr.clone());

        Self {
            current_position: location,
            open_location: None,
            locations: None,
            settings,
            connection,
            locations_error: None,
        }
    }

    fn base_group(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Location");
                    ui.label("Latitude");
                    let mut lat_text = self.current_position.latitude.to_string();
                    ui.text_edit_singleline(&mut lat_text);
                    if let Some(new_lat) = lat_text.parse().ok() {
                        self.current_position.latitude = new_lat;
                    } else if lat_text.is_empty() {
                        self.current_position.latitude = 0.0;
                    }

                    ui.label("Longitude");
                    let mut long_text = self.current_position.longitude.to_string();
                    ui.text_edit_singleline(&mut long_text);
                    if let Some(new_long) = long_text.parse().ok() {
                        self.current_position.longitude = new_long;
                    } else if long_text.is_empty() {
                        self.current_position.longitude = 0.0;
                    }

                    if ui.button("Use Current Location").clicked() {
                        let pos = public_ip_address::perform_lookup(None).ok();

                        self.current_position = Location {
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

                    ui.label("Server Address");
                    ui.text_edit_singleline(&mut self.settings.server_addr);

                    if ui.button("Connect").clicked() {
                        self.connection.connect(self.settings.server_addr.clone());
                    }

                    if self.connection.loading() {
                        ui.label("Loading...");
                    } else if let Some(err) = self
                        .connection
                        .error
                        .try_lock()
                        .ok()
                        .as_deref()
                        .cloned()
                        .flatten()
                    {
                        ui.label(RichText::new(err).color(Color32::RED));
                    } else if self
                        .connection
                        .client
                        .try_lock()
                        .ok()
                        .map(|client| client.is_some())
                        .unwrap_or_default()
                    {
                        ui.label("Connected");
                    } else {
                        ui.label("Not Connected");
                    }

                    ui.label("Max Distance");

                    ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(
                            &mut self.settings.max_distance,
                            0.0..=100.0,
                        ));
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

                    ui.add(egui::Slider::new(
                        &mut self.settings.number_to_generate,
                        0..=10,
                    ));

                    ui.label("Closeness Bias");

                    ui.add(egui::Slider::new(
                        &mut self.settings.closeness_bias,
                        -3.0..=3.0,
                    ));

                    ui.label("Minimum Rating");

                    ui.add(egui::Slider::new(
                        &mut self.settings.minimum_rating,
                        0.0..=5.0,
                    ));
                });
            });
        });

        if ui.button("Generate").clicked() {
            self.generate_options();
        }

        if let Some(err) = &self.locations_error {
            ui.label(RichText::new(err).color(Color32::RED));
        }

        if let Some(locations) = &self.locations {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    for (i, location) in locations.iter().enumerate() {
                        if location_preview(ui, location) {
                            self.open_location = Some(i);
                        }
                    }
                });
            });
        }
    }

    fn generate_options(&mut self) {
        self.locations_error = None;

        match self.connection.get_random_location(
            self.current_position.clone(),
            Distance::from(self.settings.max_distance, self.settings.distance_unit),
            self.settings.number_to_generate,
            self.settings.minimum_rating,
            self.settings.closeness_bias,
        ) {
            Ok(locations) => {
                self.locations = Some(locations);
                self.locations_error = None;
            }
            Err(err) => {
                self.locations = None;
                self.locations_error = Some(err);
            }
        }
    }
}

pub fn location_preview(ui: &mut egui::Ui, info: &common::LocationInfo) -> bool {
    ui.vertical(|ui| {
        ui.group(|ui| {
            ui.heading(&info.name);
            ui.label(&info.address);
            ui.label(format!("{} stars", info.rating));
            ui.label(format!("{} miles away", info.distance.miles()));

            if let Some(website) = info.website.as_ref() {
                if ui.link(website).clicked() {
                    if open::that(website).is_err() {
                        println!("failed to open link");
                    }
                }
            }

            ui.button("More Info").clicked()
        })
        .inner
    })
    .inner
}

pub fn location_expanded(ui: &mut egui::Ui, info: &common::LocationInfo) -> bool {
    let back = ui.button("Back").clicked();

    ui.heading(&info.name);
    ui.label(&info.address);
    ui.label(format!("{} stars", info.rating));
    ui.label(format!("{} miles away", info.distance.miles()));

    if let Some(website) = info.website.as_ref() {
        if ui.link(website).clicked() {
            if open::that(website).is_err() {
                println!("failed to open link");
            }
        }
    }

    if let Some(description) = info.description.as_ref() {
        ui.label(description);
    }

    back
}
