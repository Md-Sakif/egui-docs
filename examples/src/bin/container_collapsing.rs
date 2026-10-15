use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "CollapsingHeader",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The shortcut: closed until the user clicks it.
            ui.collapsing("Click to expand", |ui| {
                ui.label("You found me!");
            });

            // The builder gives you more options, such as starting open.
            egui::CollapsingHeader::new("Open by default")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label("Collapsing headers can be nested:");
                    ui.collapsing("A nested section", |ui| {
                        ui.label("Like a tree view.");
                    });
                });
        });
    }
}
