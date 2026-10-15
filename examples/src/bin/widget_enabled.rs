use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Enabled and visible",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    unlocked: bool,
    name: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.checkbox(&mut self.unlocked, "Unlock the form");

            ui.separator();

            // Enable or disable a single widget.
            ui.add_enabled(self.unlocked, egui::Button::new("One button"))
                .on_disabled_hover_text("Tick the checkbox first");

            // Enable or disable everything inside a block.
            ui.add_enabled_ui(self.unlocked, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.name);
                });
                let _ = ui.button("Submit");
            });

            // Hide a widget but keep its space, so nothing below it jumps around.
            ui.add_visible(self.unlocked, egui::Label::new("Thanks for unlocking!"));
            ui.label("This line never moves.");
        });
    }
}
