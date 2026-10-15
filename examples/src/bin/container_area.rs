use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Area",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("An Area is a bare floating layer. It has no background, no title bar and no border.");
        });

        // Pinned to the bottom right corner. An Area draws nothing itself,
        // so wrap the content in a Frame to give it a background.
        egui::Area::new(egui::Id::new("badge"))
            .anchor(egui::Align2::RIGHT_BOTTOM, [-12.0, -12.0])
            .show(ui.ctx(), |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.label("3 new messages");
                });
            });

        // Areas can be dragged around unless you make them immovable.
        // `default_pos` is only where the area starts. `default_size` tells egui
        // how much room to plan for before it knows the real size of the content.
        egui::Area::new(egui::Id::new("sticky_note"))
            .default_pos([40.0, 70.0])
            .default_size([140.0, 40.0])
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(255, 230, 120))
                    .inner_margin(10)
                    .corner_radius(4)
                    .show(ui, |ui| {
                        ui.colored_label(egui::Color32::BLACK, "Drag me around");
                    });
            });
    }
}
