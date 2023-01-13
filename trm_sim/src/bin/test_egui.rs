//! test for egui crate

use eframe::{self, egui, Frame};
use eframe::egui::Context;

struct TestApp {
    name: String,
    age: u32,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            name: "John".to_string(),
            age: 42,
        }
    }
}

impl eframe::App for TestApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label("This is some non-interactive text, just demonstrating labels.");
            ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Your name"));
            ui.add(egui::Slider::new(&mut self.age, 0..=130).text("age"));
            if ui.button("Click me").clicked() {
                self.name = "Clicked".to_string();
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui",
        options,
        Box::new(|_cc|
            Box::<TestApp>::default()
        )
    )
}
