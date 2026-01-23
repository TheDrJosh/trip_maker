use crate::app::App;


mod app;
mod location;
mod utils;
mod trip_advisor;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Trip Maker",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .expect("failed to launch app")
}
