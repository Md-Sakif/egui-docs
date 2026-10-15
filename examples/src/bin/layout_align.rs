use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Alignment",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Center every widget horizontally.
            ui.vertical_centered(|ui| {
                ui.heading("Centered");
                let _ = ui.button("OK");
            });

            ui.separator();

            // Left side and right side on the same row.
            ui.horizontal(|ui| {
                ui.label("Left");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let _ = ui.button("Right");
                    let _ = ui.button("Also right");
                });
            });

            ui.separator();

            // A row that wraps onto new lines when it runs out of width.
            ui.horizontal_wrapped(|ui| {
                for i in 1..=10 {
                    let _ = ui.button(format!("Item {i}"));
                }
            });

            // Fill the rest of the window from the bottom up.
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.label("Stuck to the bottom");
            });
        });
    }
}
