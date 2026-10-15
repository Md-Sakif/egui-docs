use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Table",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct Planet {
    name: &'static str,
    moons: u32,
    day_hours: f32,
}

const PLANETS: [Planet; 8] = [
    Planet {
        name: "Mercury",
        moons: 0,
        day_hours: 1407.6,
    },
    Planet {
        name: "Venus",
        moons: 0,
        day_hours: 5832.5,
    },
    Planet {
        name: "Earth",
        moons: 1,
        day_hours: 23.9,
    },
    Planet {
        name: "Mars",
        moons: 2,
        day_hours: 24.6,
    },
    Planet {
        name: "Jupiter",
        moons: 95,
        day_hours: 9.9,
    },
    Planet {
        name: "Saturn",
        moons: 146,
        day_hours: 10.7,
    },
    Planet {
        name: "Uranus",
        moons: 28,
        day_hours: 17.2,
    },
    Planet {
        name: "Neptune",
        moons: 16,
        day_hours: 16.1,
    },
];

#[derive(Default)]
struct MyApp {
    sort_by_moons: bool,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.checkbox(&mut self.sort_by_moons, "Sort by number of moons");

            let mut rows: Vec<&Planet> = PLANETS.iter().collect();
            if self.sort_by_moons {
                rows.sort_by_key(|planet| std::cmp::Reverse(planet.moons));
            }

            // A scroll area keeps the table usable when there are many rows.
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("planets")
                    .num_columns(3)
                    .striped(true)
                    .spacing([32.0, 6.0])
                    .show(ui, |ui| {
                        // Header row
                        ui.strong("Planet");
                        ui.strong("Moons");
                        ui.strong("Length of day");
                        ui.end_row();

                        for planet in rows {
                            ui.label(planet.name);
                            ui.label(planet.moons.to_string());
                            ui.label(format!("{:.1} h", planet.day_hours));
                            ui.end_row();
                        }
                    });
            });
        });
    }
}
