use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "DragValue",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    count: i32,
    price: f64,
    width: u32,
    angle: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            count: 3,
            price: 9.5,
            width: 640,
            angle: std::f32::consts::FRAC_PI_4,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Grid::new("drag_values")
                .num_columns(2)
                .show(ui, |ui| {
                    // Drag left and right to change it, or click to type a number.
                    ui.label("Count");
                    ui.add(egui::DragValue::new(&mut self.count));
                    ui.end_row();

                    // Slower dragging, a range, a prefix and two decimals.
                    ui.label("Price");
                    ui.add(
                        egui::DragValue::new(&mut self.price)
                            .speed(0.1)
                            .range(0.0..=100.0)
                            .prefix("$ ")
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Width");
                    ui.add(
                        egui::DragValue::new(&mut self.width)
                            .range(0..=4096)
                            .suffix(" px"),
                    );
                    ui.end_row();

                    // Stored in radians, shown in degrees.
                    ui.label("Angle");
                    ui.drag_angle(&mut self.angle);
                    ui.end_row();
                });
        });
    }
}
