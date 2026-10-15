use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Spinner",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // A spinner next to some text is the classic "please wait".
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Loading...");
            });

            // Bigger, and in your own color.
            ui.add(
                egui::Spinner::new()
                    .size(40.0)
                    .color(egui::Color32::from_rgb(255, 149, 0)),
            );
        });
    }
}
