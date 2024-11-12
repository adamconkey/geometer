use std::fs;
use std::collections::HashMap;


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    demo: crate::plot_demo::PlotDemo,
    paths: HashMap<String, Vec<String>>,
    selected: String,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let mut paths = HashMap::new();
        let test_names = vec![
            String::from("test_1"),
            String::from("test_2"),
            String::from("test_3"),
        ];
        paths.insert(String::from("test_file_1"), test_names);

        let selected = String::from("test_1");
        


        // Default::default()
        Self { demo: crate::plot_demo::PlotDemo::default(), paths, selected}
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

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

            // TODO in a very basic sense, I think this achieves what I want,
            // this can exist within an iterator over the folders in the test
            // results dir, the name will be a folder, and then beneath it
            // you'll have N selectable values which, when selected, sets the
            // current result to be visualized on the app. I think long-term
            // this will be unwieldy, but short term it will allow me to stand
            // up the basic infrastructure for visualizing the test results,
            // and then can add probably collapsible entries here to at least
            // make that aspect more tenable
            
            for (test_file, test_names) in self.paths.iter() {
                ui.label(test_file);
                for test_name in test_names.iter() {
                    ui.selectable_value(
                        &mut self.selected, 
                        test_name.to_string(), 
                        test_name
                    );
                }
                ui.separator();
            }
            

            


        });

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.demo.ui(ui);
        });
    }
}
