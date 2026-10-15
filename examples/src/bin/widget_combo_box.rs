use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "ComboBox",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum Language {
    #[default]
    Rust,
    Python,
    Go,
}

#[derive(Default)]
struct MyApp {
    language: Language,
    fruit: usize,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The usual way: an enum and one `selectable_value` per option.
            egui::ComboBox::from_label("Language")
                .selected_text(format!("{:?}", self.language))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.language, Language::Rust, "Rust");
                    ui.selectable_value(&mut self.language, Language::Python, "Python");
                    ui.selectable_value(&mut self.language, Language::Go, "Go");
                });

            // Choosing from a list by index.
            let fruits = ["Apple", "Banana", "Cherry"];
            egui::ComboBox::from_label("Fruit").show_index(
                ui,
                &mut self.fruit,
                fruits.len(),
                |i| fruits[i],
            );

            ui.separator();
            ui.label(format!("{:?} and {}", self.language, fruits[self.fruit]));
        });
    }
}
