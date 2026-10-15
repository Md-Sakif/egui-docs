use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Hello egui",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp))),
    )
}

struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello, world!");
            ui.label("This is my first egui app.");
        });
    }
}
