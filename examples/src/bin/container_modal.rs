use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Modal",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    confirm_open: bool,
    deleted: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.button("Delete all files").clicked() {
                self.confirm_open = true;
            }
            if self.deleted {
                ui.label("All files were deleted.");
            }
        });

        if self.confirm_open {
            // A modal darkens the rest of the app and blocks it until closed.
            let modal = egui::Modal::new(egui::Id::new("confirm_delete")).show(ui.ctx(), |ui| {
                ui.set_width(220.0);
                ui.heading("Are you sure?");
                ui.label("This cannot be undone.");
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        ui.close();
                    }
                    if ui.button("Delete").clicked() {
                        self.deleted = true;
                        ui.close();
                    }
                });
            });

            // True after `ui.close()`, a click outside the modal, or Escape.
            if modal.should_close() {
                self.confirm_open = false;
            }
        }
    }
}
