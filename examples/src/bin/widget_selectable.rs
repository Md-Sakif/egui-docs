use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Selectable label",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default, Clone, Copy, PartialEq)]
enum View {
    #[default]
    List,
    Grid,
    Map,
}

#[derive(Default)]
struct MyApp {
    view: View,
    bold: bool,
    italic: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Pick one of several values, like radio buttons that look like tabs.
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.view, View::List, "List");
                ui.selectable_value(&mut self.view, View::Grid, "Grid");
                ui.selectable_value(&mut self.view, View::Map, "Map");
            });

            ui.separator();

            ui.horizontal(|ui| {
                // `toggle_value` flips a bool for you.
                ui.toggle_value(&mut self.bold, "Bold");

                // `selectable_label` only draws. You handle the click.
                if ui.selectable_label(self.italic, "Italic").clicked() {
                    self.italic = !self.italic;
                }
            });

            let mut text = egui::RichText::new("Sample text");
            if self.bold {
                text = text.strong();
            }
            if self.italic {
                text = text.italics();
            }
            ui.label(text);
        });
    }
}
