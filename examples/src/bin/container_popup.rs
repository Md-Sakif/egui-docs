use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Popup",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    notifications: bool,
    brightness: f32,
    last_action: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            notifications: true,
            brightness: 0.8,
            last_action: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                // A menu-style popup: opens under the button and
                // closes as soon as something in it is clicked.
                let response = ui.button("Actions");
                egui::Popup::menu(&response).show(|ui| {
                    if ui.button("Duplicate").clicked() {
                        self.last_action = "Duplicate".to_owned();
                    }
                    if ui.button("Archive").clicked() {
                        self.last_action = "Archive".to_owned();
                    }
                });

                // A popup that stays open while you use the widgets inside it,
                // and closes when you click somewhere else.
                let response = ui.button("Quick settings");
                egui::Popup::from_toggle_button_response(&response)
                    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                    .show(|ui| {
                        ui.set_min_width(180.0);
                        ui.checkbox(&mut self.notifications, "Notifications");
                        ui.add(
                            egui::Slider::new(&mut self.brightness, 0.0..=1.0).text("Brightness"),
                        );
                    });
            });

            ui.label(format!("Last action: {}", self.last_action));
        });
    }
}
