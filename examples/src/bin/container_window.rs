use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Window",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    settings_open: bool,
    volume: f32,
    muted: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            settings_open: true,
            volume: 0.5,
            muted: false,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Open settings").clicked() {
                self.settings_open = true;
            }
        });

        // A window floats above the panels. `open` adds a close button
        // and hides the window while the bool is false.
        egui::Window::new("Settings")
            .open(&mut self.settings_open)
            .default_pos([40.0, 50.0])
            .default_size([260.0, 80.0])
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0).text("Volume"));
                ui.checkbox(&mut self.muted, "Mute");
            });

        // No title bar, pinned to the bottom right corner.
        egui::Window::new("Hint")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -10.0])
            .show(ui.ctx(), |ui| {
                ui.label("Drag a window by its title bar.");
            });
    }
}
