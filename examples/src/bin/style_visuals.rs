use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Custom style",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    checked: bool,
    value: f32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // `all_styles_mut` changes both the dark and the light style.
        cc.egui_ctx.all_styles_mut(|style| {
            // Colors
            let accent = egui::Color32::from_rgb(200, 80, 120);
            style.visuals.selection.bg_fill = accent;
            style.visuals.hyperlink_color = accent;

            // Rounder widgets
            let radius = egui::CornerRadius::same(10);
            style.visuals.widgets.inactive.corner_radius = radius;
            style.visuals.widgets.hovered.corner_radius = radius;
            style.visuals.widgets.active.corner_radius = radius;

            // More breathing room
            style.spacing.button_padding = egui::vec2(12.0, 6.0);
            style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        });

        Self {
            checked: true,
            value: 0.6,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("A custom look");
            ui.horizontal(|ui| {
                let _ = ui.button("Button");
                let _ = ui.selectable_label(true, "Selected");
                let _ = ui.link("A link");
            });
            ui.checkbox(&mut self.checked, "Checkbox");
            ui.add(egui::Slider::new(&mut self.value, 0.0..=1.0));
        });
    }
}
