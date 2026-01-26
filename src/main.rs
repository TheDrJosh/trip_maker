use std::time::Duration;

use tokio::runtime::Runtime;

use crate::app::App;


mod app;
mod connection;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    let _enter = rt.enter();

    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Trip Maker",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
    .expect("failed to launch app")
}
