use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Persistence",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

/// Everything in this struct is saved when the app closes
/// and restored the next time it starts.
#[derive(Default, serde::Serialize, serde::Deserialize)]
struct MyApp {
    name: String,
    visits: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load the previous state, if there is any.
        let mut app: Self = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default();

        app.visits += 1;
        app
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("You have opened this app {} times.", self.visits));
            ui.label("Close the app and start it again: your name is still here.");
        });
    }

    /// Called when the app closes, and every 30 seconds while it runs.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
