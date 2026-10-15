// On Windows, release builds would otherwise open a console window next to the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icon.png"))
        .expect("the icon is a valid PNG");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("My App")
            .with_inner_size([400.0, 300.0])
            .with_icon(icon),
        ..Default::default()
    };

    // The first argument is the app's ID. Among other things it decides
    // where saved state is stored, so pick it once and keep it.
    eframe::run_native("my-app", options, Box::new(|_cc| Ok(Box::new(MyApp))))
}

struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Ready to ship");
        });
    }
}
