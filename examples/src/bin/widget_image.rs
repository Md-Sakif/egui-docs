use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Image",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp;

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // egui cannot decode PNG or JPEG files on its own.
        // This one-time call teaches it how, using the `egui_extras` crate.
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Self
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // `include_image!` embeds the file in your executable at compile time.
            // The path is relative to this source file.
            ui.image(egui::include_image!("../../assets/landscape.png"));

            ui.horizontal(|ui| {
                // In a horizontal layout an image would shrink to the height of
                // the row, so give it an exact size. Also round the corners.
                ui.add(
                    egui::Image::new(egui::include_image!("../../assets/landscape.png"))
                        .fit_to_exact_size(egui::vec2(120.0, 75.0))
                        .corner_radius(12),
                );

                // Tinted.
                ui.add(
                    egui::Image::new(egui::include_image!("../../assets/landscape.png"))
                        .fit_to_exact_size(egui::vec2(120.0, 75.0))
                        .tint(egui::Color32::from_rgb(255, 180, 180)),
                );
            });

            // Images work inside buttons too.
            let icon = egui::include_image!("../../assets/icon.png");
            let _ = ui.add(egui::Button::image_and_text(icon, "Play"));
        });
    }
}
