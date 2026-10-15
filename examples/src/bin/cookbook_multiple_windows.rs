use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Multiple windows",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    show_inspector: bool,
    counter: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Main window");
            ui.checkbox(&mut self.show_inspector, "Show the inspector window");
            ui.label(format!("Counter: {}", self.counter));
        });

        if self.show_inspector {
            // A viewport is a real operating system window.
            // "Immediate" means it is drawn right here, inside our `ui` function,
            // so it can use `self` like any other piece of UI.
            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of("inspector"),
                egui::ViewportBuilder::default()
                    .with_title("Inspector")
                    .with_inner_size([240.0, 120.0]),
                |ui, _class| {
                    egui::CentralPanel::default().show(ui, |ui| {
                        ui.label("I am a second window.");
                        if ui.button("Increment the counter").clicked() {
                            self.counter += 1;
                        }
                    });

                    // Untick the checkbox when the user closes this window.
                    if ui.input(|i| i.viewport().close_requested()) {
                        self.show_inspector = false;
                    }
                },
            );
        }
    }
}
