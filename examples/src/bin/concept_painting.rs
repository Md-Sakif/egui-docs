use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Painting",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Reserve a 300 x 180 area and get a painter that is clipped to it.
            let (response, painter) =
                ui.allocate_painter(egui::vec2(300.0, 180.0), egui::Sense::hover());
            let rect = response.rect;

            // A rounded background.
            painter.rect_filled(rect, 8.0, egui::Color32::from_rgb(30, 40, 60));

            // A filled circle in the middle.
            painter.circle_filled(rect.center(), 40.0, egui::Color32::GOLD);

            // A diagonal line.
            painter.line_segment(
                [rect.left_bottom(), rect.right_top()],
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );

            // An outline, 10 points inside the edge.
            painter.rect_stroke(
                rect.shrink(10.0),
                4.0,
                egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
                egui::StrokeKind::Inside,
            );

            // Some text.
            painter.text(
                rect.center_top() + egui::vec2(0.0, 16.0),
                egui::Align2::CENTER_TOP,
                "Painted by hand",
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );

            // Follow the mouse while it is over the canvas.
            if let Some(pos) = response.hover_pos() {
                painter.circle_stroke(pos, 12.0, egui::Stroke::new(2.0, egui::Color32::RED));
            }
        });
    }
}
