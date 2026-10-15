use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Drag and drop",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    /// Two lists of items. Items can be dragged from one list to the other.
    columns: [Vec<&'static str>; 2],
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            columns: [vec!["Design", "Write code", "Test"], vec!["Plan"]],
        }
    }
}

/// What travels with the mouse while dragging: where the item came from.
#[derive(Clone, Copy)]
struct DragPayload {
    column: usize,
    row: usize,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Drag tasks between the lists:");

            // Filled in when something is dropped: (payload, target column).
            let mut dropped = None;

            ui.columns(2, |uis| {
                for (column, ui) in uis.iter_mut().enumerate() {
                    let title = if column == 0 { "To do" } else { "Done" };
                    let frame = egui::Frame::group(ui.style());

                    // The frame lights up while a `DragPayload` hovers over it.
                    let (_, payload) = ui.dnd_drop_zone::<DragPayload, ()>(frame, |ui| {
                        ui.set_min_size(egui::vec2(120.0, 100.0));
                        ui.strong(title);

                        for (row, item) in self.columns[column].iter().enumerate() {
                            let id = egui::Id::new(("task", column, row));
                            ui.dnd_drag_source(id, DragPayload { column, row }, |ui| {
                                ui.label(*item);
                            });
                        }
                    });

                    if let Some(payload) = payload {
                        dropped = Some((*payload, column));
                    }
                }
            });

            if let Some((from, to_column)) = dropped {
                let item = self.columns[from.column].remove(from.row);
                self.columns[to_column].push(item);
            }
        });
    }
}
