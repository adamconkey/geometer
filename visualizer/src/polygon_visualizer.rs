use egui::Response;
use egui_plot::{
    CoordinatesFormatter, Corner, Line, Plot, Points,
};
use std::collections::HashMap;

use computational_geometry::point::Point;

use crate::app::RESULT_DIR;


#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PolygonVisualizer {
    points: HashMap<String, Vec<[f64; 2]>>,
}

impl Default for PolygonVisualizer {
    fn default() -> Self {
        let mut points = HashMap::new();
        
        for file in RESULT_DIR.files() {
            let stem = String::from(file.path().file_stem().unwrap().to_str().unwrap());
            let contents = String::from(file.contents_utf8().unwrap());
            let mut point_vec: Vec<_> = serde_json::from_str::<Vec<Point>>(&contents)
                .unwrap()
                .iter()
                .map(|p: &Point| [f64::from(p.x), f64::from(p.y)])
                .collect();
            // Pushing first to end so it closes the chain, probably
            // only want to do this for line points since it
            // duplicates a vertex
            point_vec.push(point_vec.first().unwrap().clone());
            points.insert(stem, point_vec);
        }

        Self { points }
    }
}


impl PolygonVisualizer {
    pub fn ui(&mut self, ui: &mut egui::Ui, name: &String) -> Response {
        
        let mut plot = Plot::new("polygon_visualizer")
            .show_axes(true)
            .show_grid(true);
        
        plot = plot.view_aspect(1.0);
        plot = plot.data_aspect(1.0);
        plot = plot.coordinates_formatter(
            Corner::LeftBottom, 
            CoordinatesFormatter::default()
        );
        
        let points = self.points.get(name).unwrap();
        let line = Line::new(points.clone())
            .width(4.0);
        let points = Points::new(points.clone())
            .radius(8.0);

        plot.show(ui, |plot_ui| {
            plot_ui.line(line);
            plot_ui.points(points);
        }).response
    }
}