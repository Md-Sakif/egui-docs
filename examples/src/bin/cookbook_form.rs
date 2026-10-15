use eframe::egui;

fn main() -> eframe::Result {
    eframe::run_native(
        "Sign up",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    email: String,
    password: String,
    accepted: bool,
    submitted: bool,
}

impl MyApp {
    /// One message per problem. An empty list means the form is valid.
    fn errors(&self) -> Vec<&'static str> {
        let mut errors = Vec::new();
        if !self.email.contains('@') {
            errors.push("Enter a valid email address.");
        }
        if self.password.chars().count() < 8 {
            errors.push("The password needs at least 8 characters.");
        }
        if !self.accepted {
            errors.push("You must accept the terms.");
        }
        errors
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Sign up");

            egui::Grid::new("sign_up").num_columns(2).show(ui, |ui| {
                ui.label("Email");
                ui.add(egui::TextEdit::singleline(&mut self.email).hint_text("you@example.com"));
                ui.end_row();

                ui.label("Password");
                ui.add(egui::TextEdit::singleline(&mut self.password).password(true));
                ui.end_row();
            });
            ui.checkbox(&mut self.accepted, "I accept the terms");

            // Validation is just a function of the current state,
            // so it is always up to date.
            let errors = self.errors();
            for error in &errors {
                ui.colored_label(ui.visuals().error_fg_color, *error);
            }

            // The button only works once everything is valid.
            let button = egui::Button::new("Create account");
            if ui.add_enabled(errors.is_empty(), button).clicked() {
                self.submitted = true;
            }

            if self.submitted {
                ui.label(format!("Welcome, {}!", self.email));
            }
        });
    }
}
