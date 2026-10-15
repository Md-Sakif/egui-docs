use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "ScrollArea",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.strong("Vertical");
            egui::ScrollArea::vertical()
                .id_salt("fruit")
                .max_height(80.0)
                .show(ui, |ui| {
                    for i in 1..=30 {
                        ui.label(format!("Item {i}"));
                    }
                });

            ui.separator();

            ui.strong("Horizontal");
            egui::ScrollArea::horizontal()
                .id_salt("tags")
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for i in 1..=30 {
                            let _ = ui.button(format!("Tag {i}"));
                        }
                    });
                });

            ui.separator();

            // `show_rows` only builds the rows that are visible, so even
            // a million rows cost almost nothing.
            ui.strong("One million rows");
            let row_height = ui.text_style_height(&egui::TextStyle::Body);
            egui::ScrollArea::vertical()
                .id_salt("million")
                .max_height(80.0)
                .auto_shrink(false)
                .show_rows(ui, row_height, 1_000_000, |ui, row_range| {
                    for row in row_range {
                        ui.label(format!("Row {row}"));
                    }
                });
        });
    }
}
