use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Keyboard shortcuts",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

// `COMMAND` means Cmd on macOS and Ctrl everywhere else.
const SAVE: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::S);
const NEW: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::N);

#[derive(Default)]
struct MyApp {
    log: Vec<&'static str>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // `consume_shortcut` returns true once per key press, and removes the
        // key press so no other widget reacts to it as well.
        if ui.input_mut(|i| i.consume_shortcut(&SAVE)) {
            self.log.push("Saved");
        }
        if ui.input_mut(|i| i.consume_shortcut(&NEW)) {
            self.log.push("New file");
        }

        egui::CentralPanel::default().show(ui, |ui| {
            // Show the shortcut next to the button that does the same thing.
            // `format_shortcut` writes it the way the user's platform does.
            let save_button =
                egui::Button::new("Save").shortcut_text(ui.ctx().format_shortcut(&SAVE));
            if ui.add(save_button).clicked() {
                self.log.push("Saved");
            }

            let new_button = egui::Button::new("New").shortcut_text(ui.ctx().format_shortcut(&NEW));
            if ui.add(new_button).clicked() {
                self.log.push("New file");
            }

            ui.separator();
            for entry in &self.log {
                ui.label(*entry);
            }
        });
    }
}
