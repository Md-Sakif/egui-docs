use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Slider",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    volume: u32,
    opacity: f32,
    temperature: f32,
    frequency: f32,
    level: u32,
    height: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            volume: 40,
            opacity: 0.75,
            temperature: 21.5,
            frequency: 440.0,
            level: 7,
            height: 0.3,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Integers: the range decides the type of slider you get.
            ui.add(egui::Slider::new(&mut self.volume, 0..=100).text("Volume"));

            // Floats
            ui.add(egui::Slider::new(&mut self.opacity, 0.0..=1.0).text("Opacity"));

            // A unit and a fixed step size
            ui.add(
                egui::Slider::new(&mut self.temperature, -20.0..=40.0)
                    .step_by(0.5)
                    .suffix(" °C")
                    .text("Temperature"),
            );

            // A logarithmic scale for ranges that span several magnitudes
            ui.add(
                egui::Slider::new(&mut self.frequency, 20.0..=20_000.0)
                    .logarithmic(true)
                    .suffix(" Hz")
                    .text("Frequency"),
            );

            // No number box, and a filled track
            ui.add(
                egui::Slider::new(&mut self.level, 0..=10)
                    .show_value(false)
                    .trailing_fill(true),
            );

            // A vertical slider
            ui.add(egui::Slider::new(&mut self.height, 0.0..=1.0).vertical());
        });
    }
}
