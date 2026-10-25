use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    egui::Frame::new()
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.heading("Settings");
            ui.label("Edit your settings here.");
            ui.separator();
            ui.label("TODO: make real settings")
        });
}
