use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Tooltip",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The one you need most of the time.
            ui.button("Hover me").on_hover_text("A plain text tooltip");

            // A tooltip can contain any UI.
            ui.button("Rich tooltip").on_hover_ui(|ui| {
                ui.heading("Tooltips are little UIs");
                ui.label("Put labels, images or anything else in here.");
                ui.code("ui.code(\"even code\")");
            });

            // Explain why something is disabled.
            ui.add_enabled(false, egui::Button::new("Disabled"))
                .on_disabled_hover_text("You need to log in first");

            // Tooltips work on every widget, not only on buttons.
            ui.label("A label with a tooltip")
                .on_hover_text_at_pointer("This one follows the mouse pointer");
        });
    }
}
