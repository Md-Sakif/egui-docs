use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Clipboard",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    code: String,
    copied: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            code: "EGUI-2026-ROCKS".to_owned(),
            copied: false,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Your invite code:");
            ui.horizontal(|ui| {
                ui.code(&self.code);
                if ui.button("Copy").clicked() {
                    // Put the text on the system clipboard.
                    ui.ctx().copy_text(self.code.clone());
                    self.copied = true;
                }
            });
            if self.copied {
                ui.label("Copied to the clipboard.");
            }
        });
    }
}
