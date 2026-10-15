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
    /// The text shown in the display.
    display: String,
    /// The left-hand side of the pending operation.
    accumulator: f64,
    /// The operator that is waiting for its right-hand side.
    pending: Option<&'static str>,
    /// True right after an operator or "=": the next digit starts a new number.
    start_new: bool,
}

impl Default for Calculator {
    fn default() -> Self {
        Self {
            display: "0".to_owned(),
            accumulator: 0.0,
            pending: None,
            start_new: false,
        }
    }
}

impl Calculator {
    fn press(&mut self, key: &'static str) {
        match key {
            "AC" => *self = Self::default(),
            "DEL" => {
                self.display.pop();
                if self.display.is_empty() || self.display == "-" {
                    self.display = "0".to_owned();
                }
            }
            "±" => self.set_value(-self.value()),
            "%" => self.set_value(self.value() / 100.0),
            "+" | "−" | "×" | "÷" => {
                if self.start_new {
                    // Pressed right after "=" or another operator:
                    // continue from whatever is on the display.
                    self.accumulator = self.value();
                } else {
                    self.calculate();
                }
                self.pending = Some(key);
                self.start_new = true;
            }
            "=" => {
                self.calculate();
                self.pending = None;
                self.start_new = true;
            }
            "." => {
                if self.start_new {
                    self.display = "0".to_owned();
                    self.start_new = false;
                }
                if !self.display.contains('.') {
                    self.display.push('.');
                }
            }
            digit => {
                if self.start_new || self.display == "0" {
                    self.display.clear();
                    self.start_new = false;
                }
                self.display.push_str(digit);
            }
        }
    }

    /// The number currently on the display.
    fn value(&self) -> f64 {
        self.display.parse().unwrap_or(0.0)
    }

    fn set_value(&mut self, value: f64) {
        self.display = format_number(value);
    }

    /// Apply the pending operator to the accumulator and the display.
    fn calculate(&mut self) {
        let rhs = self.value();
        let result = match self.pending {
            Some("+") => self.accumulator + rhs,
            Some("−") => self.accumulator - rhs,
            Some("×") => self.accumulator * rhs,
            Some("÷") => self.accumulator / rhs,
            _ => rhs,
        };
        self.accumulator = result;
        self.set_value(result);
    }
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "Error".to_owned();
    }
    // Hide floating point noise such as 0.30000000000000004.
    let rounded = (value * 1e10).round() / 1e10;
    if rounded == 0.0 {
        "0".to_owned()
    } else {
        rounded.to_string()
    }
}

impl eframe::App for Calculator {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                ui.label(egui::RichText::new(&self.display).size(36.0));
            });
            ui.add_space(8.0);

            egui::Grid::new("keypad")
                .spacing([6.0, 6.0])
                .show(ui, |ui| {
                    for row in KEYS {
                        for key in row {
                            let button = egui::Button::new(key);
                            if ui.add_sized([56.0, 44.0], button).clicked() {
                                self.press(key);
                            }
                        }
                        ui.end_row();
                    }
                });
        });
    }
}
