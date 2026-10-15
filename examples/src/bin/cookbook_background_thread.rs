use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Background work",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

use std::sync::mpsc;
use std::time::Duration;

#[derive(Default)]
struct MyApp {
    /// How far the job has come, from 0 to 100.
    percent: u32,
    /// The receiving end of the channel, while a job is running.
    progress: Option<mpsc::Receiver<u32>>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Collect any progress reports that arrived since the last frame.
        // `try_iter` never blocks, so the UI stays responsive.
        if let Some(progress) = &self.progress {
            for percent in progress.try_iter() {
                self.percent = percent;
            }
            if self.percent == 100 {
                self.progress = None;
            }
        }

        egui::CentralPanel::default().show(ui, |ui| {
            let running = self.progress.is_some();

            if ui
                .add_enabled(!running, egui::Button::new("Start a slow job"))
                .clicked()
            {
                let (sender, receiver) = mpsc::channel();
                self.progress = Some(receiver);
                self.percent = 0;

                // The context is cheap to clone and can be used from any thread.
                let ctx = ui.ctx().clone();

                std::thread::spawn(move || {
                    for percent in 1..=100 {
                        std::thread::sleep(Duration::from_millis(10)); // "work"
                        if sender.send(percent).is_err() {
                            return; // The app was closed.
                        }
                        // Wake up the UI so it shows the new progress.
                        ctx.request_repaint();
                    }
                });
            }

            ui.add(egui::ProgressBar::new(self.percent as f32 / 100.0).show_percentage());

            if running {
                ui.label("Working... the window stays responsive.");
            } else if self.percent == 100 {
                ui.label("Done!");
            }
        });
    }
}
