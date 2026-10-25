use eframe::egui;

mod pages;
use pages::package;
use pages::settings;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    let packages = package::get_all_pacman_packages();

    eframe::run_native(
        "PacHelp",
        options,
        Box::new(move |_creation_context| Ok(Box::new(PacHelp {
            packages,
            ..Default::default()
        }))),
    )
}

#[derive(Default)]
pub struct PacHelp {
    pub current_page: String,
    pub packages: Vec<String>,
    pub search: String,
}

impl eframe::App for PacHelp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let pages = vec![
            "Packages",
            "Settings"
        ];
        let button_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0 - 12.0;
        
        ui.horizontal(|ui| {
            egui::Frame::new()
                .inner_margin(12.0)
                .show(ui, |ui| {
                    for page in pages {
                        if ui
                            .add_sized([button_width, 44.0], egui::Button::new(page))
                            .clicked()
                        {
                            if self.current_page == page {
                                println!("Not reloading page. Still at the same page.")
                            } else {
                                self.current_page = page.to_string();
                                println!("Switching to: {page}")
                            }
                        }
                    }
                })
        });

        if self.current_page == "Packages" {
            package::show(ui, &self.packages, &mut self.search);
        } else if self.current_page == "Settings" {
            settings::show(ui)
        } else {
            egui::Area::new(egui::Id::unique("empty_page"))
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(egui::RichText::new("Welcome to PacHelp!").strong().size(25.0));
                        ui.separator();
                        ui.label("Your home to install, update, or remove pacman and AUR packages.");
                        ui.label("Fuck using the CLI! We're better off using GUI.");
                    })
                });
        }
    }
}

