use egui::Response;

use egui_plot::{
    CoordinatesFormatter, Corner, Line, Plot, Points,
};

// ----------------------------------------------------------------------------

#[derive(PartialEq, Eq, serde::Deserialize, serde::Serialize)]
enum Panel {
    TestPolygon,
}

impl Default for Panel {
    fn default() -> Self {
        Self::TestPolygon
    }
}

// ----------------------------------------------------------------------------

#[derive(Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PlotDemo {
    polygon_demo: PolygonDemo,
    open_panel: Panel,
}

impl PlotDemo {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        
        match self.open_panel {
            Panel::TestPolygon => {
                self.polygon_demo.ui(ui);
            }
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
struct PolygonDemo {}

impl Default for PolygonDemo {
    fn default() -> Self {
        Self {}
    }
}

impl PolygonDemo {
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
    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        
        let mut plot = Plot::new("polygon_demo")
            .show_axes(true)
            .show_grid(true);
        
        plot = plot.view_aspect(1.0);
        plot = plot.data_aspect(1.0);
        plot = plot.coordinates_formatter(
            Corner::LeftBottom, 
            CoordinatesFormatter::default()
        );
        
        plot.show(ui, |plot_ui| {
            plot_ui.line(self.line());
            plot_ui.points(self.points());
        }).response
    }
}
