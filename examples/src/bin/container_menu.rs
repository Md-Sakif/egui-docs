use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Menus",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    last_action: String,
    show_grid: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    // Clicking a button in a menu closes the menu.
                    if ui.button("New").clicked() {
                        self.last_action = "New".to_owned();
                    }
                    if ui
                        .add(egui::Button::new("Save").shortcut_text("Ctrl+S"))
                        .clicked()
                    {
                        self.last_action = "Save".to_owned();
                    }

                    ui.separator();

                    // A menu button inside a menu becomes a sub-menu.
                    ui.menu_button("Export", |ui| {
                        if ui.button("As PNG").clicked() {
                            self.last_action = "Export as PNG".to_owned();
                        }
                        if ui.button("As PDF").clicked() {
                            self.last_action = "Export as PDF".to_owned();
                        }
                    });

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_grid, "Show grid");
                });
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(format!("Last action: {}", self.last_action));
            ui.label(format!("Show grid: {}", self.show_grid));
        });
    }
}
