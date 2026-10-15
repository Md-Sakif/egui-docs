use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 160.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Counter",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    count: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Counter");
            ui.horizontal(|ui| {
                if ui.button("−").clicked() {
                    self.count -= 1;
                }
                ui.label(self.count.to_string());
                if ui.button("+").clicked() {
                    self.count += 1;
                }
            });
        });
    }
}
