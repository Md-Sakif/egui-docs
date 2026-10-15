use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Frame",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // A frame built from scratch: background, border, corners and padding.
            egui::Frame::new()
                .fill(egui::Color32::from_rgb(40, 110, 220))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(20, 60, 140)))
                .corner_radius(10)
                .inner_margin(12)
                .show(ui, |ui| {
                    ui.colored_label(egui::Color32::WHITE, "A custom frame");
                });

            // `group` is the quick way to draw a box around related widgets.
            ui.group(|ui| {
                ui.label("A group");
                let _ = ui.button("with a button");
            });

            // Ready-made frames that follow the current theme.
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.label("Frame::popup has a shadow");
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.label("Frame::canvas is for drawing areas");
            });
        });
    }
}
