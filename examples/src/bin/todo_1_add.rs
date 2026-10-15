use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([360.0, 420.0]),
        ..Default::default()
    };
    eframe::run_native(
        "To-do",
        options,
        Box::new(|_cc| Ok(Box::<TodoApp>::default())),
    )
}

struct Task {
    title: String,
    done: bool,
}

#[derive(Default)]
struct TodoApp {
    tasks: Vec<Task>,
    /// What the user is currently typing.
    new_task: String,
}

impl eframe::App for TodoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("To-do");

            ui.horizontal(|ui| {
                let text_edit = egui::TextEdit::singleline(&mut self.new_task)
                    .hint_text("What needs to be done?")
                    .desired_width(240.0);
                let response = ui.add(text_edit);

                // A single-line text edit loses focus when you press Enter.
                let pressed_enter =
                    response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

                if ui.button("Add").clicked() || pressed_enter {
                    let title = self.new_task.trim();
                    if !title.is_empty() {
                        self.tasks.push(Task {
                            title: title.to_owned(),
                            done: false,
                        });
                        self.new_task.clear();
                    }
                    // Keep typing without reaching for the mouse.
                    response.request_focus();
                }
            });

            ui.separator();

            for task in &mut self.tasks {
                ui.checkbox(&mut task.done, task.title.as_str());
            }
        });
    }
}
