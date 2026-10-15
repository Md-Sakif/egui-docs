use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Input",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    count: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Keyboard: was this key pressed since the last frame?
        if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.count += 1;
        }
        if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.count -= 1;
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Press the up and down arrow keys:");
            ui.heading(self.count.to_string());

            ui.separator();

            // Modifier keys that are held down right now.
            let modifiers = ui.input(|i| i.modifiers);
            ui.label(format!("Shift held: {}", modifiers.shift));

            // Where is the mouse pointer?
            match ui.input(|i| i.pointer.hover_pos()) {
                Some(pos) => ui.label(format!("Pointer at ({:.0}, {:.0})", pos.x, pos.y)),
                None => ui.label("Pointer is outside the window"),
            };
        });
    }
}
