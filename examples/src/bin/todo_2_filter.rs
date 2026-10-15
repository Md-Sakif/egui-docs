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

#[derive(Default, Clone, Copy, PartialEq)]
enum Filter {
    #[default]
    All,
    Active,
    Done,
}

#[derive(Default)]
struct TodoApp {
    tasks: Vec<Task>,
    new_task: String,
    filter: Filter,
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
                    response.request_focus();
                }
            });

            // Three mutually exclusive "tabs" that all edit `self.filter`.
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.filter, Filter::All, "All");
                ui.selectable_value(&mut self.filter, Filter::Active, "Active");
                ui.selectable_value(&mut self.filter, Filter::Done, "Done");
            });

            ui.separator();

            // We can't remove a task while we are looping over the list,
            // so remember which one to delete and do it afterwards.
            let mut delete = None;

            for (index, task) in self.tasks.iter_mut().enumerate() {
                let visible = match self.filter {
                    Filter::All => true,
                    Filter::Active => !task.done,
                    Filter::Done => task.done,
                };
                if !visible {
                    continue;
                }

                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.done, task.title.as_str());
                    if ui.small_button("Delete").clicked() {
                        delete = Some(index);
                    }
                });
            }

            if let Some(index) = delete {
                self.tasks.remove(index);
            }

            ui.separator();

            ui.horizontal(|ui| {
                let left = self.tasks.iter().filter(|task| !task.done).count();
                ui.label(format!("{left} left"));
                if ui.button("Clear completed").clicked() {
                    self.tasks.retain(|task| !task.done);
                }
            });
        });
    }
}
