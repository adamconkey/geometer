use egui::Response;
use egui_plot::{
    CoordinatesFormatter, Corner, Line, Plot, Points,
};
use std::collections::HashMap;
use std::fs;

use computational_geometry::point::Point;


#[derive(Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PolygonDemo {
    paths: HashMap<String, String>
}


impl Default for PolygonDemo {
    fn default() -> Self {
        Self { paths: HashMap::new() }
    }
}


impl PolygonDemo {

    pub fn new(paths: HashMap<String, String>) -> Self {
        Self { paths }
    }

    fn points(&self) -> Points {
        Points::new(vec![
            [0.0, 0.0],
            [10.0, 0.0],
            [10.0, 10.0],
            [5.0, 5.0],
            [0.0, 10.0],
            [0.0, 0.0],
        ])
        .radius(10.0)
    }

    fn line(&self) -> Line {
        Line::new(vec![
            [0.0, 0.0],
            [10.0, 0.0],
            [10.0, 10.0],
            [5.0, 5.0],
            [0.0, 10.0],
            [0.0, 0.0],
        ])
        .width(4.0)
    } 
}

impl PolygonDemo {
    pub fn ui(&mut self, ui: &mut egui::Ui, name: &String) -> Response {
        
        let mut plot = Plot::new("polygon_demo")
            .show_axes(true)
            .show_grid(true);
        
        plot = plot.view_aspect(1.0);
        plot = plot.data_aspect(1.0);
        plot = plot.coordinates_formatter(
            Corner::LeftBottom, 
            CoordinatesFormatter::default()
        );
        
        // TODO move this logic into a function, should also
        // create these objects on instance creation so that
        // this isn't done dynamically, will ensure it doesn't
        // crash while it's running due to file issues

        let str_contents = self.paths.get(name).unwrap();

        let mut points: Vec<_> = serde_json::from_str::<Vec<Point>>(str_contents)
            .unwrap()
            .iter()
            .map(|p: &Point| [f64::from(p.x), f64::from(p.y)])
            .collect();

        // TODO Pushing first to end so it closes the chain, probably
        // only want to do this for line points
        points.push(points.first().unwrap().clone());

        let line = Line::new(points.clone())
            .width(4.0);
        let points = Points::new(points)
            .radius(8.0);

        plot.show(ui, |plot_ui| {
            plot_ui.line(line);
            plot_ui.points(points);
        }).response
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::RESULT_DIR;

    #[test]
    fn test() {
        let file = RESULT_DIR.get_file("polygon_1.json").unwrap();
        let str_contents = String::from(file.contents_utf8().unwrap());

        let points: Vec<_> = serde_json::from_str::<Vec<Point>>(&str_contents)
            .unwrap()
            .iter()
            .map(|p: &Point| [f64::from(p.x), f64::from(p.y)])
            .collect();
        println!("POINTS: {:?}", points);
    }

}