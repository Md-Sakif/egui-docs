use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Layout basics",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // By default widgets are stacked top to bottom.
            ui.label("First");
            ui.label("Second");
            let _ = ui.button("Third");

            ui.separator();

            // `horizontal` puts everything inside it on one row.
            ui.horizontal(|ui| {
                ui.label("Side");
                let _ = ui.button("by");
                let _ = ui.button("side");
            });

            ui.separator();

            // Layouts can be nested: two columns inside a row.
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Column A");
                    let _ = ui.button("A1");
                    let _ = ui.button("A2");
                });
                ui.vertical(|ui| {
                    ui.label("Column B");
                    let _ = ui.button("B1");
                    let _ = ui.button("B2");
                });
            });
        });
    }
}
