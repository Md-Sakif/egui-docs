use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Stopwatch",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    running: bool,
    elapsed: f64,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.running {
            // `stable_dt` is the time since the previous frame, in seconds.
            self.elapsed += ui.input(|i| i.stable_dt) as f64;

            // egui only repaints when something happens (a click, a key press...).
            // Ask for another frame so the stopwatch keeps ticking.
            ui.ctx().request_repaint();
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!("{:.1} s", self.elapsed));

            ui.horizontal(|ui| {
                let label = if self.running { "Stop" } else { "Start" };
                if ui.button(label).clicked() {
                    self.running = !self.running;
                }
                if ui.button("Reset").clicked() {
                    self.elapsed = 0.0;
                }
            });
        });
    }
}
