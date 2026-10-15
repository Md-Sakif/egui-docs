use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Spacing",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Widgets get a little space");
            ui.label("between them automatically.");

            ui.add_space(20.0);
            ui.label("add_space(20.0) pushed me down.");

            ui.separator();

            ui.label("A list:");
            ui.indent("list", |ui| {
                ui.label("indented");
                ui.label("lines");
            });

            ui.separator();

            // Change the spacing for just this block.
            ui.scope(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(24.0, 2.0);
                ui.horizontal(|ui| {
                    let _ = ui.button("Wide");
                    let _ = ui.button("gaps");
                });
                ui.label("tight");
                ui.label("rows");
            });
        });
    }
}
