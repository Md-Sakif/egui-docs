use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Hyperlink",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    clicks: u32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Shows the URL itself and opens it in the browser when clicked.
            ui.hyperlink("https://www.egui.rs");

            // The same, with your own text.
            ui.hyperlink_to("The egui website", "https://www.egui.rs");

            // On the web, open the page in a new tab.
            ui.add(
                egui::Hyperlink::from_label_and_url("The egui API docs", "https://docs.rs/egui")
                    .open_in_new_tab(true),
            );

            ui.separator();

            // `link` only looks like a hyperlink. You decide what a click does.
            if ui.link("A link that runs your own code").clicked() {
                self.clicks += 1;
            }
            ui.label(format!("Clicked {} times", self.clicks));
        });
    }
}
