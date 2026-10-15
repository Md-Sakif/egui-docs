use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Text styles",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp;

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Every piece of text uses one of five named text styles.
        // Change their fonts here to resize all text in the app.
        cc.egui_ctx.all_styles_mut(|style| {
            use egui::{FontId, TextStyle};
            style.text_styles = [
                (TextStyle::Heading, FontId::proportional(28.0)),
                (TextStyle::Body, FontId::proportional(16.0)),
                (TextStyle::Button, FontId::proportional(16.0)),
                (TextStyle::Monospace, FontId::monospace(15.0)),
                (TextStyle::Small, FontId::proportional(11.0)),
            ]
            .into();
        });

        Self
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Heading, 28 points");
            ui.label("Body text, 16 points");
            let _ = ui.button("Button text, 16 points");
            ui.monospace("Monospace, 15 points");
            ui.small("Small text, 11 points");
        });
    }
}
