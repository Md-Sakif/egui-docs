use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Texture",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    texture: Option<egui::TextureHandle>,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // Create the texture once and keep the handle.
            // The texture is freed when the handle is dropped.
            let texture = self.texture.get_or_insert_with(|| {
                let size = [64, 64];
                let mut pixels = Vec::with_capacity(size[0] * size[1]);
                for y in 0..size[1] {
                    for x in 0..size[0] {
                        pixels.push(egui::Color32::from_rgb(x as u8 * 4, y as u8 * 4, 160));
                    }
                }
                let image = egui::ColorImage::new(size, pixels);
                ui.ctx()
                    .load_texture("gradient", image, egui::TextureOptions::LINEAR)
            });

            ui.horizontal(|ui| {
                // At its natural size of 64 x 64 points...
                ui.image(&*texture);

                // ...or stretched.
                ui.add(egui::Image::new(&*texture).fit_to_exact_size(egui::vec2(160.0, 96.0)));
            });
        });
    }
}
