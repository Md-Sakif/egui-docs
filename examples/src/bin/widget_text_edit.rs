use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "TextEdit",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    name: String,
    email: String,
    password: String,
    pin: String,
    notes: String,
    source: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Ferris".to_owned(),
            email: String::new(),
            password: "hunter2".to_owned(),
            pin: "1234".to_owned(),
            notes: "Several lines\nof text.".to_owned(),
            source: "fn main() {\n    println!(\"Hello\");\n}".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The shortcut for a one-line text field.
            ui.text_edit_singleline(&mut self.name);

            // Greyed-out hint text that shows while the field is empty.
            ui.add(egui::TextEdit::singleline(&mut self.email).hint_text("you@example.com"));

            // Hide what is typed.
            ui.add(egui::TextEdit::singleline(&mut self.password).password(true));

            // Limit the length and the width.
            ui.add(
                egui::TextEdit::singleline(&mut self.pin)
                    .char_limit(4)
                    .desired_width(60.0),
            );

            // Several lines.
            ui.add(
                egui::TextEdit::multiline(&mut self.notes)
                    .desired_rows(3)
                    .desired_width(f32::INFINITY),
            );

            // Monospace, and Tab inserts a tab instead of moving focus.
            ui.add(
                egui::TextEdit::multiline(&mut self.source)
                    .code_editor()
                    .desired_rows(3)
                    .desired_width(f32::INFINITY),
            );
        });
    }
}
