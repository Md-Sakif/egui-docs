use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Context menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    last_action: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            let response = ui.button("Right-click me");

            // Any widget's response can carry a context menu.
            response.context_menu(|ui| {
                if ui.button("Copy").clicked() {
                    self.last_action = "Copy".to_owned();
                }
                if ui.button("Paste").clicked() {
                    self.last_action = "Paste".to_owned();
                }
                ui.separator();
                if ui.button("Delete").clicked() {
                    self.last_action = "Delete".to_owned();
                }
            });

            ui.label(format!("Last action: {}", self.last_action));
        });
    }
}
