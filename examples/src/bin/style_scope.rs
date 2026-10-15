use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Scoped style",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Normal text");
            let _ = ui.button("Normal button");

            // Style changes made inside `scope` end when the scope ends.
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(220, 60, 60));
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                ui.spacing_mut().button_padding = egui::vec2(16.0, 8.0);

                ui.label("Red monospace text");
                let _ = ui.button("Red monospace button");
            });

            ui.label("Back to normal");
        });
    }
}
