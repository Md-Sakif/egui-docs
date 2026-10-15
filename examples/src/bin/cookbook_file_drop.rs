use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "File drop",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    dropped: Vec<String>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Files that were dropped onto the window since the last frame.
        let new_files = ui.input(|i| i.raw.dropped_files.clone());
        for file in new_files {
            // The full path on desktop. On the web it is only the file name.
            let mut entry = file.path().display().to_string();

            // On desktop the contents can be read right away.
            if let Ok(bytes) = file.bytes() {
                entry += &format!(" ({} bytes)", bytes.len());
            }
            self.dropped.push(entry);
        }

        // Is the user dragging files over the window right now?
        let hovering = ui.input(|i| !i.raw.hovered_files.is_empty());

        egui::CentralPanel::default().show(ui, |ui| {
            if hovering {
                ui.heading("Drop them!");
            } else {
                ui.heading("Drag files onto this window");
            }

            ui.separator();
            for name in &self.dropped {
                ui.label(name);
            }
        });
    }
}
