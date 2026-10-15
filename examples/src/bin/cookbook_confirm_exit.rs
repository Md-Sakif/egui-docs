use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Confirm exit",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    text: String,
    show_confirmation: bool,
    allowed_to_close: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Type something, then try to close the window.");
            ui.text_edit_multiline(&mut self.text);
        });

        // Did the user click the window's close button (or press Alt+F4)?
        let close_requested = ui.input(|i| i.viewport().close_requested());
        let unsaved_changes = !self.text.is_empty();

        if close_requested && unsaved_changes && !self.allowed_to_close {
            // Not yet! Keep the window open and ask first.
            ui.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_confirmation = true;
        }

        if self.show_confirmation {
            let modal = egui::Modal::new(egui::Id::new("confirm_exit")).show(ui.ctx(), |ui| {
                ui.heading("Quit without saving?");
                ui.label("Your text will be lost.");
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Keep editing").clicked() {
                        ui.close();
                    }
                    if ui.button("Quit").clicked() {
                        self.allowed_to_close = true;
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
            if modal.should_close() {
                self.show_confirmation = false;
            }
        }
    }
}
