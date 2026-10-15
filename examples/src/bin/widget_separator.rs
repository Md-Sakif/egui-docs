use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Separator",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Above the line");
            ui.separator();
            ui.label("Below the line");

            // More room around the line.
            ui.add(egui::Separator::default().spacing(24.0));
            ui.label("24 points of space around the line above");

            // A shorter line.
            ui.add(egui::Separator::default().shrink(60.0));

            // Inside a horizontal layout the separator becomes vertical.
            ui.horizontal(|ui| {
                ui.label("Left");
                ui.separator();
                ui.label("Middle");
                ui.separator();
                ui.label("Right");
            });
        });
    }
}
