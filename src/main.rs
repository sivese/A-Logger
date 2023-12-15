use std::time::Duration;
use eframe::{egui, Frame, Storage};
use eframe::egui::{Context, Visuals};

struct Application {
    name : String,
    age : u32,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            name: "Doss".to_owned(),
            age: 18,
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("A-Logger");

            ui.horizontal(|ui| {
                let name_label = ui.label("Text Test");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Click each year").clicked() { self.age += 1; }

            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            //ui.image();
        });
    }
}

fn main() -> Result<(), eframe::Error>{
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport : egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "A-Logger",
        options,
        Box::new(|x| {
           Box::<Application>::default()
        }),
    )
}