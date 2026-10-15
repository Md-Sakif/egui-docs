use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Animation",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    expanded: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.checkbox(&mut self.expanded, "Expanded");

            // `t` glides from 0.0 to 1.0 over 0.4 seconds whenever `expanded`
            // flips. egui repaints by itself until the animation is finished.
            let id = ui.id().with("bar_animation");
            let t = ui.ctx().animate_bool_with_time(id, self.expanded, 0.4);

            // Use `t` for anything you like: here a width and a color.
            let width = egui::lerp(40.0..=280.0, t);
            let color = egui::Color32::from_rgb(40, 110, 220).gamma_multiply(0.4 + 0.6 * t);

            let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 24.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 6.0, color);

            ui.label(format!("t = {t:.2}"));
        });
    }
}
