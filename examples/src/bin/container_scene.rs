use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Scene",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    /// The part of the scene that is currently visible.
    scene_rect: egui::Rect,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // An invalid rect tells the scene to fit its whole content.
            scene_rect: egui::Rect::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Scroll to pan, Ctrl + scroll to zoom, or drag the background.");

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                egui::Scene::new()
                    .zoom_range(0.1..=4.0)
                    .show(ui, &mut self.scene_rect, |ui| {
                        ui.heading("A zoomable scene");
                        ui.label("Everything in here scales together.");
                        let _ = ui.button("Widgets still work");
                    });
            });
        });
    }
}
