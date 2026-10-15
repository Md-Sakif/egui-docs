use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Zoom",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Zoom the whole UI");
            ui.label("Ctrl + Plus, Ctrl + Minus and Ctrl + 0 work out of the box.");

            ui.horizontal(|ui| {
                if ui.button("Smaller").clicked() {
                    egui::gui_zoom::zoom_out(ui.ctx());
                }
                if ui.button("Bigger").clicked() {
                    egui::gui_zoom::zoom_in(ui.ctx());
                }
                if ui.button("Reset").clicked() {
                    ui.ctx().set_zoom_factor(1.0);
                }
            });

            ui.label(format!("Zoom factor: {:.1}", ui.ctx().zoom_factor()));
        });
    }
}
