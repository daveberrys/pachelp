use eframe::egui;
use std::process::Command;

pub fn get_all_pacman_packages() -> Vec<String> {
    let output = match Command::new("pacman").arg("-qQ").output() {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };
    
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn show(ui: &mut egui::Ui, packages: &[String]) {
    egui::Frame::new()
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.heading("Packages");
            ui.label("Search for a package and view its details here.");
            ui.separator();

            let row_height = ui.text_style_height(&egui::TextStyle::Button) + 12.0;
            egui::ScrollArea::vertical().show_rows(
                ui,
                row_height,
                packages.len(),
                |ui, row_range| {
                    for package in &packages[row_range] {
                        egui::Frame::new()
                            .inner_margin(4.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(package);
            
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("Install").clicked() {
                                            println!("Install clicked: {package}");
                                        }
                                        if ui.button("Details").clicked() {
                                            println!("Details clicked: {package}");
                                        }
                                    });
                                });
                            });
                    }
                },
            );
        });
}
