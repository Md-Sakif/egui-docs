//! Runs every example app headlessly with `egui_kittest`, drives it a little,
//! and saves the screenshots used by the docs to `../static/img/examples/`.
//!
//! Each example is pulled in with `include!`, so the code that is tested here
//! is byte-for-byte the code shown in the docs.
#![allow(dead_code)]

use eframe::egui;
use egui_kittest::Harness;

const OUT_DIR: &str = "../static/img/examples";

/// Render the harness and save it as `<name>.<theme>.png`.
fn save<S>(harness: &mut Harness<'_, S>, name: &str, theme: egui::Theme) {
    let theme = match theme {
        egui::Theme::Dark => "dark",
        egui::Theme::Light => "light",
    };
    std::fs::create_dir_all(OUT_DIR).unwrap();
    let image = harness.render().expect("failed to render");
    image
        .save(format!("{OUT_DIR}/{name}.{theme}.png"))
        .expect("failed to save screenshot");
}

/// Click the buttons with the given labels, one after another.
fn click_buttons<S>(harness: &mut Harness<'_, S>, labels: &[&str]) {
    use egui_kittest::kittest::Queryable as _;
    for label in labels {
        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, label)
            .click();
        harness.run_ok();
    }
}

/// Click the (only) text field and type into it.
fn type_text<S>(harness: &mut Harness<'_, S>, text: &str) {
    use egui_kittest::kittest::Queryable as _;
    harness
        .get_by_role(egui::accesskit::Role::TextInput)
        .click();
    harness.run_ok();
    harness
        .get_by_role(egui::accesskit::Role::TextInput)
        .type_text(text);
    harness.run_ok();
}

/// An in-memory stand-in for eframe's on-disk storage.
#[derive(Default)]
struct MemStorage(std::collections::HashMap<String, String>);

impl eframe::Storage for MemStorage {
    fn get_string(&self, key: &str) -> Option<String> {
        self.0.get(key).cloned()
    }
    fn set_string(&mut self, key: &str, value: String) {
        self.0.insert(key.to_owned(), value);
    }
    fn remove_string(&mut self, key: &str) {
        self.0.remove(key);
    }
    fn flush(&mut self) {}
}

/// A fake file to "drop" onto the window.
#[derive(Debug)]
struct TestFile(std::path::PathBuf, Vec<u8>);

impl egui::DroppedFile for TestFile {
    fn path(&self) -> &std::path::Path {
        &self.0
    }
    fn bytes(&self) -> Result<Vec<u8>, String> {
        Ok(self.1.clone())
    }
}

/// Run single frames until one of them emits a matching viewport command.
fn expect_viewport_command<S>(
    harness: &mut Harness<'_, S>,
    matches: impl Fn(&egui::ViewportCommand) -> bool,
) {
    for _ in 0..5 {
        harness.step();
        let output = &harness.output().viewport_output[&egui::ViewportId::ROOT];
        if output.commands.iter().any(&matches) {
            return;
        }
    }
    panic!("the expected viewport command was never sent");
}

/// Tell the app that the user asked to close the window.
fn request_close<S>(harness: &mut Harness<'_, S>) {
    harness
        .input_mut()
        .viewports
        .get_mut(&egui::ViewportId::ROOT)
        .unwrap()
        .events
        .push(egui::ViewportEvent::Close);
}

macro_rules! example {
    ($name:ident, [$w:expr, $h:expr], $ctor:expr) => {
        example!($name, [$w, $h], $ctor, |_harness, _theme| {});
    };
    ($name:ident, [$w:expr, $h:expr], $ctor:expr, |$harness:ident, $theme:ident| $interact:block) => {
        mod $name {
            include!(concat!("../src/bin/", stringify!($name), ".rs"));

            #[test]
            fn screenshot() {
                #[allow(unused_imports)]
                use eframe::egui::accesskit::Role;
                #[allow(unused_imports)]
                use egui_kittest::kittest::Queryable as _;
                for $theme in [eframe::egui::Theme::Light, eframe::egui::Theme::Dark] {
                    let mut $harness = egui_kittest::Harness::builder()
                        .with_size([$w as f32, $h as f32])
                        .with_pixels_per_point(2.0)
                        .with_theme($theme)
                        .wgpu()
                        .build_eframe($ctor);
                    $harness.run_ok();
                    $interact
                    $harness.run_ok();
                    crate::save(&mut $harness, stringify!($name), $theme);
                }
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Getting started

example!(
    first_app,
    [320, 80],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("+").click();
        harness.run();
        harness.get_by_label("+").click();
        harness.run();
        assert_eq!(harness.state().count, 2);
        harness.remove_cursor();
    }
);

example!(hello_world, [320, 70], |_cc| MyApp);

// ---------------------------------------------------------------------------
// Tutorials

example!(calc_1_layout, [262, 330], |_cc| Calculator::default());

example!(
    calc_2_logic,
    [262, 330],
    |_cc| Calculator::default(),
    |harness, _theme| {
        let cases: &[(&[&str], &str)] = &[
            (&["1", "2", "+", "3", "="], "15"),
            (&["×", "2", "="], "30"), // keep calculating with the result
            (&["AC", "1", "÷", "4", "="], "0.25"),
            (&["AC", "0", ".", "1", "+", "0", ".", "2", "="], "0.3"),
            (&["AC", "5", "÷", "0", "="], "Error"),
            (&["AC", "9", "±", "−", "1", "="], "-10"),
            (&["AC", "5", "0", "%"], "0.5"),
            (&["AC", "1", "2", "3", "DEL"], "12"),
            (&["+", "×", "3", "="], "36"), // changing your mind about the operator
            (&["AC", "2", "+", "3", "+", "4", "="], "9"), // chained operators
            (&["AC", "1", "2", "+", "3", "="], "15"),
        ];
        for (keys, expected) in cases {
            crate::click_buttons(&mut harness, keys);
            assert_eq!(
                harness.state().display,
                *expected,
                "after pressing {keys:?}"
            );
        }
        harness.remove_cursor();
    }
);

example!(
    calc_3_styled,
    [262, 330],
    |_cc| Calculator::default(),
    |harness, _theme| {
        crate::click_buttons(&mut harness, &["7", "×", "6", "="]);
        assert_eq!(harness.state().display, "42");
        harness.remove_cursor();
    }
);

example!(
    todo_1_add,
    [360, 190],
    |_cc| TodoApp::default(),
    |harness, _theme| {
        for title in ["Buy milk", "Learn egui", "Ship the app"] {
            crate::type_text(&mut harness, title);
            crate::click_buttons(&mut harness, &["Add"]);
        }
        assert_eq!(harness.state().tasks.len(), 3);
        assert!(harness.state().new_task.is_empty());

        // Enter adds a task too.
        crate::type_text(&mut harness, "Press Enter");
        harness.key_press(eframe::egui::Key::Enter);
        harness.run();
        assert_eq!(harness.state().tasks.len(), 4);
        assert_eq!(harness.state().tasks[3].title, "Press Enter");

        harness.get_by_label("Buy milk").click();
        harness.run();
        assert!(harness.state().tasks[0].done);
        harness.remove_cursor();
    }
);

example!(
    todo_2_filter,
    [360, 200],
    |_cc| TodoApp::default(),
    |harness, _theme| {
        for title in ["Buy milk", "Learn egui", "Ship the app", "Oops"] {
            crate::type_text(&mut harness, title);
            crate::click_buttons(&mut harness, &["Add"]);
        }
        // Delete the last task.
        harness.get_all_by_label("Delete").last().unwrap().click();
        harness.run();
        assert_eq!(harness.state().tasks.len(), 3);

        harness.get_by_label("Buy milk").click();
        harness.run();
        harness.get_by_label("2 left");

        // The "Active" filter hides the finished task.
        harness.get_by_label("Active").click();
        harness.run();
        assert!(harness.query_by_label("Buy milk").is_none());
        harness.get_by_label("Learn egui");

        // The "Done" filter shows only the finished task.
        harness.get_by_label("Done").click();
        harness.run();
        harness.get_by_label("Buy milk");
        assert!(harness.query_by_label("Learn egui").is_none());

        harness.get_by_label("All").click();
        harness.run();
        harness.remove_cursor();
    }
);

// ---------------------------------------------------------------------------
// Core concepts

example!(
    concept_immediate,
    [320, 100],
    |_cc| MyApp::default(),
    |harness, _theme| {
        assert!(harness.query_by_label("Here are the details!").is_none());
        harness.get_by_label("Show details").click();
        harness.run();
        harness.get_by_label("Here are the details!");
        harness.remove_cursor();
    }
);

example!(
    concept_app,
    [340, 170],
    |cc| MyApp::new(cc),
    |harness, _theme| {
        crate::click_buttons(&mut harness, &["Increment"]);
        assert_eq!(harness.state().age, 43);
        harness.get_by_label("Hello 'Arthur', age 43");
        harness.remove_cursor();
    }
);

example!(
    concept_response,
    [380, 130],
    |_cc| MyApp::default(),
    |harness, _theme| {
        let button = "Click, double-click or right-click me";
        harness.get_by_label(button).click_secondary();
        harness.run();
        assert_eq!(harness.state().last_event, "Right-clicked");
        harness.get_by_label("A clickable label").click();
        harness.run();
        assert_eq!(harness.state().last_event, "Label clicked");
        harness.get_by_label(button).click();
        harness.run();
        assert_eq!(harness.state().last_event, "Clicked");
    }
);

example!(
    concept_ids,
    [300, 150],
    |_cc| MyApp::default(),
    |harness, _theme| {
        // Opening the first "Details" must not open the others.
        harness.get_all_by_label("Details").next().unwrap().click();
        harness.run();
        harness.get_by_label("All about Alice");
        assert!(harness.query_by_label("All about Bob").is_none());
        assert!(harness.query_by_label("All about Dave").is_none());
        harness.remove_cursor();
    }
);

example!(
    concept_input,
    [320, 125],
    |_cc| MyApp::default(),
    |harness, _theme| {
        for _ in 0..3 {
            harness.key_press(eframe::egui::Key::ArrowUp);
            harness.run();
        }
        harness.key_press(eframe::egui::Key::ArrowDown);
        harness.run();
        assert_eq!(harness.state().count, 2);
        harness.hover_at(eframe::egui::pos2(200.0, 60.0));
        harness.run();
        harness.get_by_label("Pointer at (200, 60)");
    }
);

example!(
    concept_repaint,
    [240, 90],
    |_cc| MyApp::default(),
    |harness, _theme| {
        crate::click_buttons(&mut harness, &["Start"]);
        harness.run_steps(20);
        assert!(harness.state().running);
        assert!(harness.state().elapsed > 0.0);
        crate::click_buttons(&mut harness, &["Stop"]);
        let elapsed = harness.state().elapsed;
        harness.run_steps(5);
        assert_eq!(
            harness.state().elapsed,
            elapsed,
            "a stopped stopwatch must not tick"
        );
        harness.remove_cursor();
    }
);

example!(
    concept_painting,
    [332, 212],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.hover_at(eframe::egui::pos2(240.0, 140.0));
    }
);

// ---------------------------------------------------------------------------
// Layout

example!(layout_basic, [300, 190], |_cc| MyApp::default());
example!(layout_align, [340, 250], |_cc| MyApp::default());
example!(layout_columns, [420, 100], |_cc| MyApp::default());
example!(layout_spacing, [300, 225], |_cc| MyApp::default());
example!(layout_sizing, [340, 165], |_cc| MyApp::default());

// ---------------------------------------------------------------------------
// Styling

example!(
    style_theme,
    [380, 135],
    |_cc| MyApp::default(),
    |harness, theme| {
        use eframe::egui::Theme;
        crate::click_buttons(&mut harness, &["Go dark"]);
        assert_eq!(harness.ctx.theme(), Theme::Dark);
        crate::click_buttons(&mut harness, &["Go light"]);
        assert_eq!(harness.ctx.theme(), Theme::Light);
        // Leave the app in the theme this screenshot is for.
        let button = if theme == Theme::Dark {
            "Go dark"
        } else {
            "Go light"
        };
        crate::click_buttons(&mut harness, &[button]);
        harness.remove_cursor();
    }
);

example!(style_visuals, [300, 150], |cc| MyApp::new(cc));
example!(style_text, [320, 150], |cc| MyApp::new(cc));
example!(style_scope, [300, 145], |_cc| MyApp::default());

// ---------------------------------------------------------------------------
// Widgets

example!(widget_label, [340, 245], |_cc| MyApp::default());
example!(widget_rich_text, [300, 290], |_cc| MyApp::default());

example!(
    widget_hyperlink,
    [320, 125],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness
            .get_by_label("A link that runs your own code")
            .click();
        harness.run();
        assert_eq!(harness.state().clicks, 1);
        harness.get_by_label("Clicked 1 times");
    }
);

example!(
    widget_button,
    [340, 150],
    |_cc| MyApp::default(),
    |harness, _theme| {
        crate::click_buttons(&mut harness, &["Click me", "Click me", "Click me"]);
        assert_eq!(harness.state().clicks, 3);
        // A disabled button must not react.
        harness.get_by_label("Disabled").click();
        harness.run();
        harness.remove_cursor();
    }
);

example!(
    widget_checkbox,
    [360, 170],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("I accept the terms").click();
        harness.run();
        assert!(harness.state().terms);

        // "All fruit" ticks both, then unticking one makes it indeterminate.
        harness.get_by_label("All fruit").click();
        harness.run();
        assert!(harness.state().apples && harness.state().pears);
        harness.get_by_label("Pears").click();
        harness.run();
        assert!(harness.state().apples && !harness.state().pears);
        harness.remove_cursor();
    }
);

example!(
    widget_radio,
    [300, 135],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("Large").click();
        harness.run();
        assert_eq!(harness.state().size, Size::Large);
        harness.get_by_label("Selected: Large");
        harness.get_by_label("Muted").click();
        harness.run();
        assert!(harness.state().muted);
        harness.get_by_label("Sound on").click();
        harness.run();
        assert!(!harness.state().muted);
        harness.remove_cursor();
    }
);

example!(
    widget_selectable,
    [300, 100],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("Grid").click();
        harness.run();
        assert!(harness.state().view == View::Grid);
        harness.get_by_label("Bold").click();
        harness.run();
        assert!(harness.state().bold);
        harness.get_by_label("Italic").click();
        harness.run();
        assert!(harness.state().italic);
        harness.get_by_label("Italic").click();
        harness.run();
        assert!(!harness.state().italic);
        harness.remove_cursor();
    }
);

example!(widget_slider, [360, 260], |_cc| MyApp::default());
example!(widget_drag_value, [240, 110], |_cc| MyApp::default());

example!(
    widget_text_edit,
    [340, 250],
    |_cc| MyApp::default(),
    |harness, _theme| {
        // Typing into the PIN field must stop at four characters.
        let is_pin = |node: &egui_kittest::kittest::AccessKitNode<'_>| {
            node.role() == Role::TextInput && node.value().as_deref() == Some("1234")
        };
        harness.get_by(is_pin).click();
        harness.run();
        harness.get_by(is_pin).type_text("56");
        harness.run();
        assert_eq!(harness.state().pin, "1234");
        harness.remove_cursor();
    }
);

example!(
    widget_combo_box,
    [300, 130],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness
            .get_by_role_and_label(Role::ComboBox, "Language")
            .click();
        harness.run();
        harness.get_by_label("Python").click();
        harness.run();
        assert_eq!(harness.state().language, Language::Python);

        harness
            .get_by_role_and_label(Role::ComboBox, "Fruit")
            .click();
        harness.run();
        harness.get_by_label("Cherry").click();
        harness.run();
        assert_eq!(harness.state().fruit, 2);
        harness.get_by_label("Python and Cherry");

        // A second screenshot with the list open.
        harness
            .get_by_role_and_label(Role::ComboBox, "Language")
            .click();
        harness.run();
        harness.remove_cursor();
        crate::save(&mut harness, "widget_combo_box_open", theme);
        harness.key_press(eframe::egui::Key::Escape);
        harness.run();
    }
);

example!(widget_progress_bar, [340, 135], |_cc| MyApp::default());
example!(widget_spinner, [240, 95], |_cc| MyApp::default());
example!(widget_separator, [340, 145], |_cc| MyApp::default());
example!(widget_image, [352, 335], |cc| MyApp::new(cc));
example!(widget_image_texture, [270, 112], |_cc| MyApp::default());
example!(widget_color_picker, [300, 290], |_cc| MyApp::default());

example!(
    widget_custom,
    [300, 95],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness
            .get_all_by_role(Role::CheckBox)
            .next()
            .unwrap()
            .click();
        harness.run();
        assert!(harness.state().wifi);
        assert!(!harness.state().bluetooth);
        harness.get_by_label("Wi-Fi is on, Bluetooth is off");
        harness.remove_cursor();
    }
);

example!(
    widget_enabled,
    [340, 170],
    |_cc| MyApp::default(),
    |harness, theme| {
        // While locked, the form ignores clicks and typing.
        harness.get_by_label("Submit").click();
        harness.run();
        crate::save(&mut harness, "widget_enabled_locked", theme);
        harness.get_by_label("Unlock the form").click();
        harness.run();
        harness.get_by_label("Thanks for unlocking!");
        harness.remove_cursor();
    }
);

// ---------------------------------------------------------------------------
// Containers

example!(container_central_panel, [420, 140], |_cc| MyApp::default());

example!(
    container_panel,
    [480, 220],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness.get_by_label("Projects").click();
        harness.run();
        assert_eq!(harness.state().page, "Projects");
        harness.remove_cursor();
        crate::save(&mut harness, "container_panel_open", theme);

        // Collapse the sidebar.
        harness.get_by_label("Sidebar").click();
        harness.run();
        assert!(!harness.state().sidebar_open);
        assert!(harness.query_by_label("Settings").is_none());
        harness.remove_cursor();
    }
);

example!(
    container_window,
    [400, 220],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("Mute").click();
        harness.run();
        assert!(harness.state().muted);
        // Closing and reopening the window.
        harness.get_by_label("Close window").click();
        harness.run();
        assert!(!harness.state().settings_open);
        assert!(harness.query_by_label("Mute").is_none());
        crate::click_buttons(&mut harness, &["Open settings"]);
        assert!(harness.state().settings_open);
        harness.get_by_label("Mute");
        harness.remove_cursor();
    }
);

example!(container_area, [380, 170], |_cc| MyApp::default());
example!(container_frame, [320, 210], |_cc| MyApp::default());
example!(container_scroll_area, [320, 300], |_cc| MyApp::default());

example!(
    container_grid,
    [360, 115],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("Send me updates").click();
        harness.run();
        assert!(!harness.state().newsletter);
        harness.get_by_label("Send me updates").click();
        harness.run();
        harness.remove_cursor();
    }
);

example!(
    container_collapsing,
    [320, 150],
    |_cc| MyApp::default(),
    |harness, _theme| {
        assert!(harness.query_by_label("You found me!").is_none());
        harness.get_by_label("Collapsing headers can be nested:");
        harness.get_by_label("Click to expand").click();
        harness.run();
        harness.get_by_label("You found me!");
        harness.remove_cursor();
    }
);

example!(
    container_modal,
    [380, 220],
    |_cc| MyApp::default(),
    |harness, theme| {
        // Cancel closes the modal without deleting.
        crate::click_buttons(&mut harness, &["Delete all files"]);
        assert!(harness.state().confirm_open);
        crate::click_buttons(&mut harness, &["Cancel"]);
        assert!(!harness.state().confirm_open);
        assert!(!harness.state().deleted);

        // Escape closes it too.
        crate::click_buttons(&mut harness, &["Delete all files"]);
        harness.key_press(eframe::egui::Key::Escape);
        harness.run();
        assert!(!harness.state().confirm_open);

        // Screenshot with the modal open, then confirm.
        crate::click_buttons(&mut harness, &["Delete all files"]);
        harness.remove_cursor();
        harness.run();
        crate::save(&mut harness, "container_modal_open", theme);
        crate::click_buttons(&mut harness, &["Delete"]);
        assert!(harness.state().deleted);
        assert!(!harness.state().confirm_open);
        harness.remove_cursor();
    }
);

example!(
    container_menu,
    [380, 200],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness.get_by_label("File").click();
        harness.run();
        harness.get_by_label_contains("Save").click();
        harness.run();
        assert_eq!(harness.state().last_action, "Save");
        // Clicking an item closed the menu.
        assert!(harness.query_by_label("New").is_none());

        harness.get_by_label("View").click();
        harness.run();
        harness.get_by_label("Show grid").click();
        harness.run();
        assert!(harness.state().show_grid);
        // A checkbox in a menu closes the menu too.
        assert!(harness.query_by_label("Show grid").is_none());

        // Sub-menu.
        harness.get_by_label("File").click();
        harness.run();
        harness.get_by_label_contains("Export").hover();
        harness.run();
        harness.run_steps(10);
        crate::save(&mut harness, "container_menu_open", theme);
        harness.get_by_label("As PDF").click();
        harness.run();
        assert_eq!(harness.state().last_action, "Export as PDF");
        harness.remove_cursor();
    }
);

example!(
    container_context_menu,
    [300, 150],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness.get_by_label("Right-click me").click_secondary();
        harness.run();
        harness.get_by_label("Paste").click();
        harness.run();
        assert_eq!(harness.state().last_action, "Paste");
        assert!(harness.query_by_label("Copy").is_none());

        harness.get_by_label("Right-click me").click_secondary();
        harness.run();
        let _ = theme;
    }
);

example!(
    container_popup,
    [360, 150],
    |_cc| MyApp::default(),
    |harness, theme| {
        crate::click_buttons(&mut harness, &["Actions"]);
        harness.get_by_label("Archive").click();
        harness.run();
        assert_eq!(harness.state().last_action, "Archive");
        assert!(harness.query_by_label("Duplicate").is_none());

        // The settings popup stays open while its widgets are used.
        crate::click_buttons(&mut harness, &["Quick settings"]);
        harness.get_by_label("Notifications").click();
        harness.run();
        assert!(!harness.state().notifications);
        harness.get_by_label("Notifications");
        harness.remove_cursor();
        let _ = theme;
    }
);

example!(
    container_tooltip,
    [360, 190],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness.get_by_label("Hover me").hover();
        harness.run_steps(30);
        harness.get_by_label("A plain text tooltip");
        crate::save(&mut harness, "container_tooltip_text", theme);
        harness.get_by_label("Rich tooltip").hover();
        harness.run_steps(30);
        harness.get_by_label("Tooltips are little UIs");
    }
);

example!(
    container_sides,
    [320, 130],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_all_by_label("Remove").nth(1).unwrap().click();
        harness.run();
        assert_eq!(harness.state().removed, ["photo.png"]);
        assert!(harness.query_by_label("photo.png").is_none());
        harness.remove_cursor();
    }
);

example!(container_resize, [340, 150], |_cc| MyApp::default());
example!(container_scene, [400, 220], |_cc| MyApp::default());

// ---------------------------------------------------------------------------
// Cookbook

example!(cookbook_custom_font, [340, 120], |cc| MyApp::new(cc));

example!(
    cookbook_shortcuts,
    [300, 135],
    |_cc| MyApp::default(),
    |harness, _theme| {
        use eframe::egui::{Key, Modifiers};
        harness.key_press_modifiers(Modifiers::COMMAND, Key::S);
        harness.run();
        harness.key_press_modifiers(Modifiers::COMMAND, Key::N);
        harness.run();
        assert_eq!(harness.state().log, ["Saved", "New file"]);
        // A plain "S" is not the shortcut.
        harness.key_press(Key::S);
        harness.run();
        assert_eq!(harness.state().log.len(), 2);
        harness
            .get_by(|node| {
                node.role() == Role::Button && node.label().is_some_and(|l| l.starts_with("Save"))
            })
            .click();
        harness.run();
        assert_eq!(harness.state().log, ["Saved", "New file", "Saved"]);
        harness.remove_cursor();
    }
);

example!(
    cookbook_tabs,
    [360, 100],
    |_cc| MyApp::default(),
    |harness, _theme| {
        harness.get_by_label("Everything at a glance.");
        harness.get_by_label("Activity").click();
        harness.run();
        harness.get_by_label("Nothing has happened yet.");
        harness.get_by_label("Settings").click();
        harness.run();
        harness.get_by_label("Enable notifications").click();
        harness.run();
        assert!(harness.state().notifications);
        harness.remove_cursor();
    }
);

example!(
    cookbook_confirm_exit,
    [380, 220],
    |_cc| MyApp::default(),
    |harness, theme| {
        use eframe::egui::ViewportCommand;

        // With nothing typed, closing is not intercepted.
        crate::request_close(&mut harness);
        harness.step();
        assert!(!harness.state().show_confirmation);

        harness.get_by_role(Role::MultilineTextInput).click();
        harness.run();
        harness
            .get_by_role(Role::MultilineTextInput)
            .type_text("Dear diary");
        harness.run();

        // Now a close request is cancelled and the modal appears.
        crate::request_close(&mut harness);
        crate::expect_viewport_command(&mut harness, |cmd| *cmd == ViewportCommand::CancelClose);
        harness.run();
        assert!(harness.state().show_confirmation);
        harness.remove_cursor();
        harness.run();
        crate::save(&mut harness, "cookbook_confirm_exit_open", theme);

        crate::click_buttons(&mut harness, &["Keep editing"]);
        assert!(!harness.state().show_confirmation);

        // Asking again and choosing "Quit" really closes the window.
        crate::request_close(&mut harness);
        harness.run();
        harness.get_by_label("Quit").click();
        crate::expect_viewport_command(&mut harness, |cmd| *cmd == ViewportCommand::Close);
        assert!(harness.state().allowed_to_close);
    }
);

example!(
    cookbook_background_thread,
    [340, 100],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness.get_by_label("Start a slow job").click();
        harness.step();
        harness.step();
        let mut saw_partial_progress = false;
        for _ in 0..500 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            harness.step();
            let percent = harness.state().percent;
            if (40..70).contains(&percent) && !saw_partial_progress {
                saw_partial_progress = true;
                harness.remove_cursor();
                harness.step();
                crate::save(&mut harness, "cookbook_background_thread_running", theme);
            }
            if percent == 100 {
                break;
            }
        }
        assert!(saw_partial_progress, "progress should arrive bit by bit");
        assert_eq!(harness.state().percent, 100);
        harness.run();
        harness.get_by_label("Done!");
    }
);

example!(
    cookbook_persistence,
    [420, 100],
    |cc| MyApp::new(cc),
    |harness, _theme| {
        use eframe::App as _;
        assert_eq!(harness.state().visits, 1);
        crate::type_text(&mut harness, "Ferris");

        // Save, then start a "second run" of the app from what was saved.
        let mut storage = crate::MemStorage::default();
        harness.state_mut().save(&mut storage);

        let mut cc = eframe::CreationContext::_new_kittest(eframe::egui::Context::default());
        cc.storage = Some(&storage);
        let second_run = MyApp::new(&cc);
        assert_eq!(second_run.name, "Ferris");
        assert_eq!(second_run.visits, 2);
        harness.remove_cursor();
    }
);

example!(
    cookbook_multiple_windows,
    [460, 250],
    |_cc| MyApp::default(),
    |harness, _theme| {
        assert!(harness.query_by_label("I am a second window.").is_none());
        harness.get_by_label("Show the inspector window").click();
        harness.run();
        harness.get_by_label("I am a second window.");
        crate::click_buttons(
            &mut harness,
            &["Increment the counter", "Increment the counter"],
        );
        assert_eq!(harness.state().counter, 2);
        harness.get_by_label("Counter: 2");

        // The test harness has no real second window, so egui embeds the viewport
        // as a window inside the first one. Drag it aside so both are visible.
        let window = harness.get_by_label("Inspector").rect();
        let title = window.center_top() + eframe::egui::vec2(0.0, 12.0);
        let target = title + eframe::egui::vec2(170.0, 90.0);
        harness.hover_at(title);
        harness.run();
        harness.drag_at(title);
        harness.run();
        harness.hover_at(title + eframe::egui::vec2(80.0, 40.0));
        harness.run();
        harness.hover_at(target);
        harness.run();
        harness.drop_at(target);
        harness.run();
        harness.remove_cursor();
    }
);

example!(
    cookbook_drag_and_drop,
    [340, 170],
    |_cc| MyApp::default(),
    |harness, theme| {
        let from = harness.get_by_label("Write code").rect().center();
        let to = harness.get_by_label("Plan").rect().center();
        harness.hover_at(from);
        harness.run();
        harness.drag_at(from);
        harness.run();
        harness.hover_at(from + (to - from) * 0.5);
        harness.run();
        let to = to + eframe::egui::vec2(20.0, 40.0); // still inside the "Done" box
        harness.hover_at(to);
        harness.run();
        crate::save(&mut harness, "cookbook_drag_and_drop_dragging", theme);
        harness.drop_at(to);
        harness.run();
        assert_eq!(harness.state().columns[0], ["Design", "Test"]);
        assert_eq!(harness.state().columns[1], ["Plan", "Write code"]);
        harness.remove_cursor();
    }
);

example!(
    cookbook_file_drop,
    [400, 120],
    |_cc| MyApp::default(),
    |harness, _theme| {
        for (path, size) in [
            ("/home/ferris/photo.png", 2048),
            ("/home/ferris/notes.txt", 12),
        ] {
            let file = crate::TestFile(path.into(), vec![0; size]);
            harness
                .input_mut()
                .dropped_files
                .push(std::sync::Arc::new(file));
            harness.run();
        }
        assert_eq!(
            harness.state().dropped,
            [
                "/home/ferris/photo.png (2048 bytes)",
                "/home/ferris/notes.txt (12 bytes)"
            ]
        );
    }
);

example!(
    cookbook_animation,
    [320, 100],
    |_cc| MyApp::default(),
    |harness, theme| {
        harness.get_by_label("t = 0.00");
        crate::save(&mut harness, "cookbook_animation_collapsed", theme);
        harness.get_by_label("Expanded").click();
        harness.run(); // runs until the animation has finished
        harness.get_by_label("t = 1.00");
        harness.remove_cursor();
    }
);

example!(
    cookbook_form,
    [380, 215],
    |_cc| MyApp::default(),
    |harness, theme| {
        // An invalid form cannot be submitted.
        harness.get_by_label("Enter a valid email address.");
        harness.get_by_label("Create account").click();
        harness.run();
        assert!(!harness.state().submitted);
        harness.remove_cursor();
        harness.run();
        crate::save(&mut harness, "cookbook_form_errors", theme);

        let fields: Vec<_> = harness
            .get_all_by_role(Role::TextInput)
            .map(|n| n.rect().center())
            .collect();
        let password: Vec<_> = harness
            .get_all_by_role(Role::PasswordInput)
            .map(|n| n.rect().center())
            .collect();
        assert_eq!((fields.len(), password.len()), (1, 1));
        harness.get_by_role(Role::TextInput).click();
        harness.run();
        harness
            .get_by_role(Role::TextInput)
            .type_text("ferris@example.com");
        harness.run();
        harness.get_by_role(Role::PasswordInput).click();
        harness.run();
        harness
            .get_by_role(Role::PasswordInput)
            .type_text("correct horse");
        harness.run();
        harness.get_by_label("I accept the terms").click();
        harness.run();
        assert!(harness.state().errors().is_empty());
        crate::click_buttons(&mut harness, &["Create account"]);
        assert!(harness.state().submitted);
        harness.get_by_label("Welcome, ferris@example.com!");
        harness.remove_cursor();
    }
);

example!(
    cookbook_table,
    [340, 250],
    |_cc| MyApp::default(),
    |harness, _theme| {
        let top = |harness: &egui_kittest::Harness<'_, MyApp>, name: &str| {
            harness.get_by_label(name).rect().top()
        };
        assert!(top(&harness, "Mercury") < top(&harness, "Saturn"));
        harness.get_by_label("Sort by number of moons").click();
        harness.run();
        assert!(top(&harness, "Saturn") < top(&harness, "Jupiter"));
        assert!(top(&harness, "Jupiter") < top(&harness, "Mercury"));
        harness.remove_cursor();
    }
);

example!(
    cookbook_clipboard,
    [300, 90],
    |_cc| MyApp::default(),
    |harness, _theme| {
        use eframe::egui::OutputCommand;
        harness.get_by_label("Copy").click();
        let mut copied = None;
        for _ in 0..5 {
            harness.step();
            for command in &harness.output().platform_output.commands {
                if let OutputCommand::CopyText(text) = command {
                    copied = Some(text.clone());
                }
            }
        }
        assert_eq!(copied.as_deref(), Some("EGUI-2026-ROCKS"));
        assert!(harness.state().copied);
        harness.remove_cursor();
    }
);

example!(
    cookbook_zoom,
    [420, 120],
    |_cc| MyApp::default(),
    |harness, _theme| {
        crate::click_buttons(&mut harness, &["Bigger"]);
        assert!(harness.ctx.zoom_factor() > 1.0);
        crate::click_buttons(&mut harness, &["Reset"]);
        assert_eq!(harness.ctx.zoom_factor(), 1.0);
        crate::click_buttons(&mut harness, &["Smaller"]);
        assert!(harness.ctx.zoom_factor() < 1.0);
        crate::click_buttons(&mut harness, &["Reset"]);
        harness.remove_cursor();
    }
);

// ---------------------------------------------------------------------------
// Window options

example!(
    window_options,
    [420, 150],
    |_cc| MyApp::default(),
    |harness, _theme| {
        use eframe::egui::ViewportCommand;
        harness.get_by_label("Rename the window").click();
        crate::expect_viewport_command(&mut harness, |cmd| {
            *cmd == ViewportCommand::Title("Renamed 1 times".to_owned())
        });
        harness.get_by_label("Always on top").click();
        crate::expect_viewport_command(&mut harness, |cmd| {
            *cmd == ViewportCommand::WindowLevel(eframe::egui::WindowLevel::AlwaysOnTop)
        });
        harness.remove_cursor();
    }
);
