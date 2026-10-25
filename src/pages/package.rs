use eframe::egui;
use std::process::Command;

pub fn show(ui: &mut egui::Ui, packages: &[String], search: &mut String) {
    egui::Frame::new()
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.heading("Packages");
            ui.label("Search for a package and view details, install or remove here.");
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                if ui.button("Use AUR").clicked() {
                    println!("AUR not implemented.")
                }
            
                ui.add(
                    egui::TextEdit::singleline(search)
                        .hint_text("Search packages...")
                        .desired_width(f32::INFINITY),
                );
            });
            ui.separator();

            let query = search.to_lowercase();
            let filtered: Vec<&String> = packages
                .iter()
                .filter(|name| name.to_lowercase().contains(&query))
                .collect();

            if filtered.is_empty() {
                ui.label("No packages match your search.");
            }

            let row_height = ui.text_style_height(&egui::TextStyle::Button) + 12.0;
            egui::ScrollArea::vertical().show_rows(
                ui,
                row_height,
                filtered.len(),
                |ui, row_range| {
                    for &package in &filtered[row_range] {
                        egui::Frame::new()
                            .inner_margin(4.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(package);
            
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if ui.button("Install").clicked() {
                                            if let Err(err) = open_in_terminal(&["sudo", "pacman", "-S", package]) {
                                                eprintln!("Failed to launch terminal: {err}");
                                            }
                                        }

                                        if ui.button("Remove").clicked() {
                                            if let Err(err) = open_in_terminal(&["sudo", "pacman", "-Rs", package]) {
                                                eprintln!("Failed to launch terminal: {err}");
                                            }
                                        }
                                    });
                                });
                            });
                    }
                },
            );
        });
}

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

fn open_in_terminal(args: &[&str]) -> Result<(), String> {
    let terminals = [
        ("kitty", &["-e"][..]),
        ("alacritty", &["-e"]),
        ("konsole", &["-e"]),
        ("gnome-terminal", &["--"]),
        ("xterm", &["-e"]),
    ];

    for (terminal, flag) in terminals {
        if Command::new(terminal).args(flag).args(args).spawn().is_ok() {
            return Ok(());
        }
    }
    Err("no terminal emulator found".to_string())
}