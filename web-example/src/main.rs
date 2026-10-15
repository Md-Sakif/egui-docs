use eframe::egui;

/// The app itself is the same on the desktop and on the web.
#[derive(Default)]
struct MyApp {
    count: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello from egui!");
            ui.horizontal(|ui| {
                if ui.button("Click me").clicked() {
                    self.count += 1;
                }
                ui.label(format!("Clicked {} times", self.count));
            });
            ui.label(if cfg!(target_arch = "wasm32") {
                "Running in a web browser"
            } else {
                "Running as a desktop app"
            });
        });
    }
}

/// Desktop: `cargo run`
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    eframe::run_native(
        "Web example",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

/// Web: `trunk serve`
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Send `log` messages to the browser console.
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        // Find the <canvas> element from index.html...
        let canvas = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .get_element_by_id("the_canvas_id")
            .expect("index.html has no element with the id the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id is not a canvas");

        // ...and start the app in it.
        eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_cc| Ok(Box::<MyApp>::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
