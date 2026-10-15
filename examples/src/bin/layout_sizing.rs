use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Sizing",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    text: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Widgets are as small as their content by default.
            let _ = ui.button("Natural size");

            // An exact size:
            ui.add_sized([200.0, 40.0], egui::Button::new("Exactly 200 x 40"));

            // As wide as the space that is left:
            let full_width = ui.available_width();
            ui.add_sized([full_width, 30.0], egui::Button::new("Full width"));

            // A minimum size (the button still grows to fit longer text):
            ui.add(egui::Button::new("At least 150 wide").min_size(egui::vec2(150.0, 0.0)));

            // Text fields have their own width setting:
            ui.add(
                egui::TextEdit::singleline(&mut self.text)
                    .hint_text("A full-width text field")
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
