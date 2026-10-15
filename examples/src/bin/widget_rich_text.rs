use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "RichText",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

use egui::{Color32, RichText};

#[derive(Default)]
struct MyApp {
    checked: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label(RichText::new("Large").size(24.0));
            ui.label(RichText::new("Colored").color(Color32::from_rgb(80, 160, 240)));
            ui.label(RichText::new("Strong").strong());
            ui.label(RichText::new("Italics").italics());
            ui.label(RichText::new("Underlined").underline());
            ui.label(RichText::new("Strikethrough").strikethrough());
            ui.label(
                RichText::new("Highlighted")
                    .background_color(Color32::YELLOW)
                    .color(Color32::BLACK),
            );
            ui.label(RichText::new("Monospace").monospace());

            // Styles can be chained.
            ui.label(
                RichText::new("All together")
                    .size(18.0)
                    .strong()
                    .italics()
                    .underline()
                    .color(Color32::from_rgb(220, 80, 60)),
            );

            ui.separator();

            // RichText works wherever a widget takes text.
            let _ = ui.button(RichText::new("A big button").size(18.0));
            ui.checkbox(
                &mut self.checked,
                RichText::new("A strong checkbox").strong(),
            );

            ui.separator();

            // For several styles inside one label, build a LayoutJob.
            let font = egui::TextStyle::Body.resolve(ui.style());
            let normal = egui::TextFormat::simple(font.clone(), ui.visuals().text_color());
            let red = egui::TextFormat::simple(font, Color32::from_rgb(220, 80, 60));

            let mut job = egui::text::LayoutJob::default();
            job.append("One label, ", 0.0, normal.clone());
            job.append("two", 0.0, red);
            job.append(" colors", 0.0, normal);
            ui.label(job);
        });
    }
}
