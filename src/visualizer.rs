use crate::polygon::Polygon;

pub struct RerunVisualizer {
    rec: rerun::RecordingStream,
}


impl RerunVisualizer {
    pub fn new(name: String) -> Self {
        // TODO don't unwrap
        let rec = rerun::RecordingStreamBuilder::new(name).connect_tcp().unwrap();
        RerunVisualizer { rec }
    }

    pub fn visualize_polygon(&self, polygon: &Polygon, name: &String) {
        self.rec.log(
            format!("{}/vertices", name),
            &polygon.to_rerun_points()
                .with_radii([0.3]),
        ).unwrap();  // TODO don't unwrap

        self.rec.log(
            format!("{}/edges", name),
            &polygon.to_rerun_edges()
        ).unwrap();  // TODO don't unwrap
    }

    // TODO need to have this return Result and handle errors gracefully
    pub fn visualize_triangulation(&self, polygon: &Polygon, name: &String) {
        let triangulation = polygon.triangulation();

        let rerun_meshes = triangulation.to_rerun_meshes();

        self.visualize_polygon(&polygon, name);
        
        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(
                format!("{}/triangle_{}", name, i),
                mesh
            ).unwrap();  // TODO don't unwrap
        }
    }

    pub fn visualize_extreme_points(&self, polygon: &Polygon, name: &String) {
        let extreme_points: Vec<_> = polygon
            .extreme_points()
            .iter()
            .map(|id| polygon.get_point(id))
            .collect();
        
        // TODO will want to add vis of polygon as above, and then vis
        // the extreme vertices in some way that makes them stand out
        // more, probably mainly making them bigger but could also 
        // look into other visualization options. Should probably add
        // some helpers to clean all this up.

    }
}