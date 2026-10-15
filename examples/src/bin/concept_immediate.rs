use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Immediate mode",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    show_details: bool,
    frames: u64,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // This function runs again every time the window is repainted.
        self.frames += 1;

        egui::CentralPanel::default().show(ui, |ui| {
            ui.checkbox(&mut self.show_details, "Show details");

            // There is no "show" or "hide" call anywhere.
            // The label exists only on the frames where this line runs.
            if self.show_details {
                ui.label("Here are the details!");
            }

            ui.label(format!("ui() has run {} times", self.frames));
        });
    }
}
