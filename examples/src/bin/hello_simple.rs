use eframe::egui;

fn main() -> eframe::Result {
    // Application state lives in plain local variables.
    let mut name = String::from("World");

    eframe::run_ui_native(
        "Hello egui",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            egui::CentralPanel::default().show(ui, |ui| {
                ui.heading(format!("Hello, {name}!"));
                ui.text_edit_singleline(&mut name);
            });
        },
    )
}
