use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "IDs",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Three collapsing headers with the same title would normally share
            // one ID, and so share their open/closed state. `push_id` gives each
            // loop iteration its own ID namespace.
            for name in ["Alice", "Bob", "Carol"] {
                ui.push_id(name, |ui| {
                    ui.collapsing("Details", |ui| {
                        ui.label(format!("All about {name}"));
                    });
                });
            }

            ui.separator();

            // Many containers also let you set the ID source directly.
            egui::CollapsingHeader::new("Details")
                .id_salt("dave")
                .show(ui, |ui| {
                    ui.label("All about Dave");
                });
        });
    }
}
