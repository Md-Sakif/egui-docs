use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Panels",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    sidebar_open: bool,
    page: &'static str,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            sidebar_open: true,
            page: "Home",
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Panels are added from the outside in: the first ones get the full
        // width or height, later ones fit into what is left.
        egui::Panel::top("top_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.sidebar_open, "Sidebar");
                ui.heading("My App");
            });
        });

        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.label("Ready");
        });

        // A resizable sidebar that slides away when `sidebar_open` is false.
        egui::Panel::left("sidebar")
            .resizable(true)
            .default_size(120.0)
            .size_range(80.0..=200.0)
            .show_collapsible(ui, &mut self.sidebar_open, |ui| {
                for page in ["Home", "Projects", "Settings"] {
                    ui.selectable_value(&mut self.page, page, page);
                }
            });

        egui::Panel::right("inspector")
            .resizable(false)
            .exact_size(110.0)
            .show(ui, |ui| {
                ui.strong("Inspector");
                ui.label("Fixed width");
            });

        // Always last: the central panel takes whatever remains.
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(self.page);
            ui.label("The main content goes here.");
        });
    }
}
