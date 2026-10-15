use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Custom font",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp;

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Embed the font file in the executable and register it as the
        // first choice for all proportional (non-monospace) text.
        cc.egui_ctx.add_font(egui::epaint::text::FontInsert::new(
            "lato",
            egui::FontData::from_static(include_bytes!("../../assets/Lato-Regular.ttf")),
            vec![egui::epaint::text::InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            }],
        ));
        Self
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("This heading uses Lato");
            ui.label("So does this label, and every button:");
            let _ = ui.button("A Lato button");
            ui.monospace("Monospace text keeps the built-in font.");
        });
    }
}
