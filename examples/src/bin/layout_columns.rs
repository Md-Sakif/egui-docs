use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Columns",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Split the available width into three equal columns.
            // `columns` is a slice with one `Ui` per column.
            ui.columns(3, |columns| {
                columns[0].heading("Left");
                columns[0].label("Every column gets the same width.");

                columns[1].heading("Middle");
                let _ = columns[1].button("A button");

                columns[2].heading("Right");
                columns[2].label("Text wraps inside its own column.");
            });
        });
    }
}
