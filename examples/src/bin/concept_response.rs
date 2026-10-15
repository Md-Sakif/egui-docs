use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Responses",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    last_event: String,
    volume: f32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Every widget returns a `Response` describing what happened to it.
            let response = ui.button("Click, double-click or right-click me");
            if response.clicked() {
                self.last_event = "Clicked".to_owned();
            }
            if response.double_clicked() {
                self.last_event = "Double-clicked".to_owned();
            }
            if response.secondary_clicked() {
                self.last_event = "Right-clicked".to_owned();
            }
            // Responses can also add behavior, such as a tooltip.
            response.on_hover_text("I am a tooltip");

            // `changed` is true when the widget modified its value this frame.
            let response = ui.add(egui::Slider::new(&mut self.volume, 0.0..=10.0).text("volume"));
            if response.changed() {
                self.last_event = format!("Volume changed to {:.1}", self.volume);
            }

            // Labels don't react to clicks unless you ask them to.
            let label = egui::Label::new("A clickable label").sense(egui::Sense::click());
            if ui.add(label).clicked() {
                self.last_event = "Label clicked".to_owned();
            }

            ui.separator();
            ui.label(format!("Last event: {}", self.last_event));
        });
    }
}
