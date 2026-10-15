use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Label",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("A plain label");
            ui.heading("A heading");
            ui.strong("Strong text");
            ui.weak("Weak text");
            ui.small("Small text");
            ui.monospace("Monospace text");
            ui.code("inline_code()");
            ui.colored_label(egui::Color32::from_rgb(220, 80, 60), "Colored text");

            ui.separator();

            // Long text wraps by default...
            ui.label("Long text wraps onto the next line when it reaches the edge of the window.");

            // ...but you can cut it off with an ellipsis instead.
            ui.add(
                egui::Label::new("This label is truncated instead of wrapping onto a new line.")
                    .truncate(),
            );

            // Label text can be selected with the mouse unless you turn that off.
            ui.add(egui::Label::new("This text cannot be selected").selectable(false));
        });
    }
}
