use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Button",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    clicks: u32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The button you will use 90% of the time.
            if ui.button("Click me").clicked() {
                self.clicks += 1;
            }
            ui.label(format!("Clicked {} times", self.clicks));

            ui.separator();

            ui.horizontal(|ui| {
                let _ = ui.small_button("Small");
                ui.add(egui::Button::new("Selected").selected(true));
                ui.add(egui::Button::new("No frame").frame(false));
                ui.add_enabled(false, egui::Button::new("Disabled"));
            });

            ui.horizontal(|ui| {
                let blue = egui::Color32::from_rgb(40, 110, 220);
                let white = egui::Color32::WHITE;
                ui.add(egui::Button::new(egui::RichText::new("Filled").color(white)).fill(blue));
                ui.add(egui::Button::new("Outlined").stroke(egui::Stroke::new(1.5, blue)));
                ui.add(egui::Button::new("Rounded").corner_radius(12.0));
            });

            ui.horizontal(|ui| {
                ui.add(egui::Button::new("Save").shortcut_text("Ctrl+S"));
                ui.add(egui::Button::new("Big").min_size(egui::vec2(100.0, 36.0)));
            });
        });
    }
}
