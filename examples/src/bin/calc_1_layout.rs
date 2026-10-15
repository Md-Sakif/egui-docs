use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([262.0, 330.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(Calculator::default()))),
    )
}

const KEYS: [[&str; 4]; 5] = [
    ["AC", "±", "%", "÷"],
    ["7", "8", "9", "×"],
    ["4", "5", "6", "−"],
    ["1", "2", "3", "+"],
    ["0", ".", "DEL", "="],
];

struct Calculator {
    display: String,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            display: "0".to_owned(),
        }
    }
}

impl eframe::App for Calculator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // The display: one big, right-aligned number.
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                ui.label(egui::RichText::new(&self.display).size(36.0));
            });
            ui.add_space(8.0);

            // The keypad: a grid with one button per key.
            egui::Grid::new("keypad")
                .spacing([6.0, 6.0])
                .show(ui, |ui| {
                    for row in KEYS {
                        for key in row {
                            ui.add_sized([56.0, 44.0], egui::Button::new(key));
                        }
                        ui.end_row();
                    }
                });
        });
    }
}
