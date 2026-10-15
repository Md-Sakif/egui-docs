use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Grid",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
    newsletter: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Ferris".to_owned(),
            age: 10,
            newsletter: true,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Each widget goes into the next cell. `end_row` starts a new row.
            // Columns are as wide as their widest cell.
            egui::Grid::new("profile_form")
                .num_columns(2)
                .spacing([24.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.name);
                    ui.end_row();

                    ui.label("Age");
                    ui.add(egui::DragValue::new(&mut self.age).range(0..=150));
                    ui.end_row();

                    ui.label("Newsletter");
                    ui.checkbox(&mut self.newsletter, "Send me updates");
                    ui.end_row();
                });
        });
    }
}
