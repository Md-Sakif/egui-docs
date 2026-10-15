use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Theme",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    checked: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Theme");

            // A ready-made "System / Dark / Light" switch.
            egui::widgets::global_theme_preference_buttons(ui);

            // Or change the theme from your own code.
            ui.horizontal(|ui| {
                if ui.button("Go dark").clicked() {
                    ui.ctx().set_theme(egui::Theme::Dark);
                }
                if ui.button("Go light").clicked() {
                    ui.ctx().set_theme(egui::Theme::Light);
                }
                if ui.button("Follow the system").clicked() {
                    ui.ctx().set_theme(egui::ThemePreference::System);
                }
            });

            ui.separator();
            ui.label(format!("The active theme is {:?}", ui.ctx().theme()));
            ui.checkbox(&mut self.checked, "A checkbox to look at");
        });
    }
}
