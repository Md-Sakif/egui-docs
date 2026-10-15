use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Custom widget",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

/// A toggle switch. Use it like any built-in widget: `ui.add(toggle(&mut value))`.
fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        // 1. Decide how big the widget is: twice as wide as it is tall.
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

        // 2. Reserve that space, and ask egui to detect clicks on it.
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        // 3. React to the click.
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        // Tell screen readers (and UI tests) what this is.
        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
        });

        // 4. Paint, but only if the widget is actually on screen.
        if ui.is_rect_visible(rect) {
            // A value that glides from 0.0 to 1.0 when `on` changes.
            let how_on = ui.ctx().animate_bool_responsive(response.id, *on);

            // Colors that match the theme and the hovered/pressed state.
            let visuals = ui.style().interact_selectable(&response, *on);

            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter().rect(
                rect,
                radius,
                visuals.bg_fill,
                visuals.bg_stroke,
                egui::StrokeKind::Inside,
            );

            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        // 5. Return the response so callers can use `.changed()`, tooltips, and so on.
        response
    }
}

#[derive(Default)]
struct MyApp {
    wifi: bool,
    bluetooth: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.wifi));
                ui.label("Wi-Fi");
            });
            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.bluetooth))
                    .on_hover_text("Custom widgets get tooltips for free");
                ui.label("Bluetooth");
            });
            ui.label(format!(
                "Wi-Fi is {}, Bluetooth is {}",
                if self.wifi { "on" } else { "off" },
                if self.bluetooth { "on" } else { "off" },
            ));
        });
    }
}
