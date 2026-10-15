use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "CentralPanel",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Start from the standard panel look and make the margin wider.
        let frame = egui::Frame::central_panel(ui.style()).inner_margin(24);

        egui::CentralPanel::default().frame(frame).show(ui, |ui| {
            ui.heading("CentralPanel");
            ui.label("The central panel fills all the space the other panels left over.");
            ui.label("This one has a 24 point margin instead of the default 8.");
        });
    }
}
