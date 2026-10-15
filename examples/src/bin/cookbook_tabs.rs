use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Tabs",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default, Clone, Copy, PartialEq)]
enum Tab {
    #[default]
    Overview,
    Activity,
    Settings,
}

#[derive(Default)]
struct MyApp {
    tab: Tab,
    notifications: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // The tab bar: a row of selectable labels in a top panel.
        egui::Panel::top("tabs").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Overview, "Overview");
                ui.selectable_value(&mut self.tab, Tab::Activity, "Activity");
                ui.selectable_value(&mut self.tab, Tab::Settings, "Settings");
            });
        });

        // The page: draw only the selected tab.
        egui::CentralPanel::default().show(ui, |ui| match self.tab {
            Tab::Overview => {
                ui.heading("Overview");
                ui.label("Everything at a glance.");
            }
            Tab::Activity => {
                ui.heading("Activity");
                ui.label("Nothing has happened yet.");
            }
            Tab::Settings => {
                ui.heading("Settings");
                ui.checkbox(&mut self.notifications, "Enable notifications");
            }
        });
    }
}
