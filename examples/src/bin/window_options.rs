use eframe::egui;

fn main() -> eframe::Result {
    // Decode a PNG that is embedded in the executable.
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icon.png"))
        .expect("the icon is a valid PNG");

    let options = eframe::NativeOptions {
        // Everything about the window itself is set on the viewport.
        viewport: egui::ViewportBuilder::default()
            .with_title("Window options")
            .with_inner_size([420.0, 200.0])
            .with_min_inner_size([320.0, 160.0])
            .with_icon(icon),
        // Open in the middle of the screen.
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "window_options",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    fullscreen: bool,
    always_on_top: bool,
    renames: u32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Change the window while it runs");

            // Viewport commands are requests to the operating system's window.
            if ui.checkbox(&mut self.fullscreen, "Fullscreen").changed() {
                ui.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
            }

            if ui
                .checkbox(&mut self.always_on_top, "Always on top")
                .changed()
            {
                let level = if self.always_on_top {
                    egui::WindowLevel::AlwaysOnTop
                } else {
                    egui::WindowLevel::Normal
                };
                ui.send_viewport_cmd(egui::ViewportCommand::WindowLevel(level));
            }

            if ui.button("Rename the window").clicked() {
                self.renames += 1;
                let title = format!("Renamed {} times", self.renames);
                ui.send_viewport_cmd(egui::ViewportCommand::Title(title));
            }

            if ui.button("Close").clicked() {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
