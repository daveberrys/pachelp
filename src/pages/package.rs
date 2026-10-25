use eframe::egui;
use std::process::Command;

pub fn show(ui: &mut egui::Ui, packages: &mut Vec<String>, search: &mut String) {
    egui::Frame::new()
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.heading("Packages");
            ui.label("Search for a package and view details, install or remove here.");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("Fetch AUR").clicked() {
                    let aur_packages = get_aur_package(search);
                    packages.extend(aur_packages)
                }

                if ui.button("Update all packages").clicked() {
                    if let Err(err) = open_in_terminal(&["sudo", "pacman", "-Syu",]) {
                        eprintln!("Failed to launch terminal: {err}");
                    }
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
                            .inner_margin(2.0)
                            .show(ui, |ui| {
                                egui::Frame::new()
                                    .fill(egui::Color32::from_rgb(25, 25, 25))
                                    .inner_margin(6.0)
                                    .corner_radius(8)
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.colored_label(egui::Color32::from_rgb(255, 255, 255), package);
                    
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                if ui.button("Install").clicked() {
                                                    if let Some(helper) = get_paru_or_yay() {
                                                        if let Err(err) = open_in_terminal(&[helper.as_str(), "-Sy", package]) {
                                                            eprintln!("Failed to launch terminal: {err}");
                                                        }
                                                    } else {
                                                        eprintln!("Neither paru or yay is found. Falling back to pacman.");
                                                        if let Err(err) = open_in_terminal(&["sudo", "pacman", "-Sy", package]) {
                                                            eprintln!("Failed to launch terminal: {err}");
                                                        }
                                                    }
                                                }
        
                                                if ui.button("Remove").clicked() {
                                                    if let Some(helper) = get_paru_or_yay() {
                                                        if let Err(err) = open_in_terminal(&[helper.as_str(), "-Rs", package]) {
                                                            eprintln!("Failed to launch terminal: {err}");
                                                        }
                                                    } else {
                                                        eprintln!("Neither paru or yay is found. Falling back to pacman.");
                                                        if let Err(err) = open_in_terminal(&["sudo", "pacman", "-Rs", package]) {
                                                            eprintln!("Failed to launch terminal: {err}");
                                                        }
                                                    }
                                                }
                                            });
                                        });
                                    })
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

fn get_aur_package(package: &str) -> Vec<String> {
    let Some(helper) = get_paru_or_yay() else { return Vec::new(); };
    let output = match Command::new(helper).args(["-Ss", "--color", "never", package]).output() {
        Ok(output) => output,
        Err(_) => return Vec::new(),
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.starts_with(char::is_whitespace))
        .filter_map(|line| {
            let first = line.split_whitespace().next()?;
            Some(first.rsplit('/').next()?.to_owned())
        })
        .collect()
}

fn get_paru_or_yay() -> Option<String> {
    for program in ["paru", "yay"] {
        if Command::new(program)
            .arg("--version")
            .output()
            .is_ok()
        {
            return Some(program.to_string());
        }
    }

    None
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