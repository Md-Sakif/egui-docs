use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "ProgressBar",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    progress: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { progress: 0.4 }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Progress goes from 0.0 (empty) to 1.0 (full).
            ui.add(egui::ProgressBar::new(self.progress));

            ui.add(egui::ProgressBar::new(self.progress).show_percentage());

            // Your own text, and a moving highlight to show work is going on.
            ui.add(
                egui::ProgressBar::new(self.progress)
                    .text("Downloading...")
                    .animate(true),
            );

            // A thin, short, green bar.
            ui.add(
                egui::ProgressBar::new(self.progress)
                    .desired_width(160.0)
                    .desired_height(6.0)
                    .fill(egui::Color32::from_rgb(60, 180, 90)),
            );

            ui.separator();
            ui.add(egui::Slider::new(&mut self.progress, 0.0..=1.0).text("progress"));
        });
    }
}
