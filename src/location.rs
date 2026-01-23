use eframe::egui;

#[derive(Debug, Clone, Copy)]
pub struct LocationOption {}

impl LocationOption {
    pub fn preview(&self, ui: &mut egui::Ui) -> bool {
        ui.vertical(|ui| {
            ui.group(|ui| {
                ui.heading("test");
                ui.button("More Info").clicked()
            })
            .inner
        })
        .inner
    }

    pub fn expanded(&self, ui: &mut egui::Ui) -> bool {
        ui.button("Back").clicked()
    }
}
