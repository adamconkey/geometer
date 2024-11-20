use include_dir::{include_dir, Dir};
use std::collections::HashMap;


pub const RESULT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/results");


#[derive(Default)]
pub struct TemplateApp {
    demo: crate::plot_demo::PolygonDemo,
    filenames: Vec<String>,
    selected: String,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        let mut filenames = Vec::new();
        let mut paths = HashMap::new();
        
        for file in RESULT_DIR.files() {
            let stem = String::from(file.path().file_stem().unwrap().to_str().unwrap());
            // let path_str = String::from(file.path().to_str().unwrap());
            let path_str = String::from(file.contents_utf8().unwrap());
            filenames.push(stem.clone());
            paths.insert(stem, path_str);
        }

        let selected = filenames[0].clone();
        
        Self { demo: crate::plot_demo::PolygonDemo::new(paths), filenames, selected}
    }
}

impl eframe::App for TemplateApp {
    
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::SidePanel::left("test_result_browser")
        .resizable(true)
        .default_width(160.0)
        .min_width(160.0)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Test Results");
            });
            ui.separator();

            for name in self.filenames.iter() {
                ui.selectable_value(
                    &mut self.selected, 
                    name.to_string(), 
                    name
                );
            }
        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.demo.ui(ui, &self.selected);
        });
    }
}
