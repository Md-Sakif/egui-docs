use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Checkbox",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    terms: bool,
    apples: bool,
    pears: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // A checkbox edits a `bool` that you own.
            ui.checkbox(&mut self.terms, "I accept the terms");

            ui.separator();

            // A "select all" checkbox that shows a dash when only some are ticked.
            let mut all = self.apples && self.pears;
            let some = self.apples != self.pears;
            let select_all = egui::Checkbox::new(&mut all, "All fruit").indeterminate(some);
            if ui.add(select_all).changed() {
                self.apples = all;
                self.pears = all;
            }
            ui.indent("fruit", |ui| {
                ui.checkbox(&mut self.apples, "Apples");
                ui.checkbox(&mut self.pears, "Pears");
            });

            ui.separator();

            // A checkbox without a label, and a disabled one.
            ui.horizontal(|ui| {
                ui.add(egui::Checkbox::without_text(&mut self.terms));
                ui.label("<- no label, same value as the first checkbox");
            });
            let mut always_on = true;
            ui.add_enabled(false, egui::Checkbox::new(&mut always_on, "Disabled"));
        });
    }
}
