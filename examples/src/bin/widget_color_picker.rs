use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Color picker",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    accent: egui::Color32,
    background: [f32; 3],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            accent: egui::Color32::from_rgb(255, 149, 0),
            background: [0.1, 0.2, 0.4],
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // A small swatch that opens a color picker when clicked.
            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.accent);
                ui.label("Accent (a Color32)");
            });

            // The same button works with plain arrays of floats or bytes.
            ui.horizontal(|ui| {
                ui.color_edit_button_rgb(&mut self.background);
                ui.label("Background (an [f32; 3])");
            });

            ui.separator();

            // The full picker, always visible.
            egui::widgets::color_picker::color_picker_color32(
                ui,
                &mut self.accent,
                egui::widgets::color_picker::Alpha::Opaque,
            );
        });
    }
}
