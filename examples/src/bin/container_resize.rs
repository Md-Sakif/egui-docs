use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Resize",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Drag the bottom right corner of the box:");

            egui::Frame::group(ui.style()).show(ui, |ui| {
                egui::Resize::default()
                    .default_size([220.0, 80.0])
                    .min_size([120.0, 40.0])
                    .max_size([300.0, 160.0])
                    .show(ui, |ui| {
                        ui.label("The text inside wraps to fit whatever size you give the box.");
                    });
            });
        });
    }
}
