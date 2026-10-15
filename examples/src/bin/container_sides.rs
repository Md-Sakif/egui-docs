use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Sides",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    removed: Vec<&'static str>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            for file in ["report.pdf", "photo.png", "notes.txt"] {
                if self.removed.contains(&file) {
                    continue;
                }

                // The first closure is laid out from the left edge,
                // the second from the right edge.
                egui::Sides::new().show(
                    ui,
                    |ui| {
                        ui.label(file);
                    },
                    |ui| {
                        if ui.button("Remove").clicked() {
                            self.removed.push(file);
                        }
                    },
                );
                ui.separator();
            }
        });
    }
}
