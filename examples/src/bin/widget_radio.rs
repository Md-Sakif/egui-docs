use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "RadioButton",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Default)]
struct MyApp {
    size: Size,
    muted: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // `radio_value` sets `self.size` to the given value when clicked,
            // and shows as selected while `self.size` equals that value.
            ui.radio_value(&mut self.size, Size::Small, "Small");
            ui.radio_value(&mut self.size, Size::Medium, "Medium");
            ui.radio_value(&mut self.size, Size::Large, "Large");
            ui.label(format!("Selected: {:?}", self.size));

            ui.separator();

            // `radio` only draws the button. You handle the click yourself.
            ui.horizontal(|ui| {
                if ui.radio(!self.muted, "Sound on").clicked() {
                    self.muted = false;
                }
                if ui.radio(self.muted, "Muted").clicked() {
                    self.muted = true;
                }
            });
        });
    }
}
