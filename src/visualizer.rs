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
                .with_radii([5.0]),
        ).unwrap();  // TODO don't unwrap

        self.rec.log(
            format!("{}/edges", name),
            &polygon.to_rerun_edges()
                .with_colors([(3, 144, 252)])
        ).unwrap();  // TODO don't unwrap
    }

    // TODO need to have this return Result and handle errors gracefully
    pub fn visualize_triangulation(&self, polygon: &Polygon, name: &String) {
        let name = format!("{name}/triangulation");
        let triangulation = polygon.triangulation();
        let rerun_meshes = triangulation.to_rerun_meshes();

        self.visualize_polygon(polygon, &name);
        
        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(
                format!("{}/triangle_{}", &name, i),
                mesh
            ).unwrap();  // TODO don't unwrap
        }
    }

    // TODO have this return Result
    pub fn visualize_extreme_points(&self, polygon: &Polygon, name: &String) {
        let name = format!("{name}/extreme_points");
        
        self.visualize_polygon(polygon, &name);
        
        let extreme_points: Vec<_> = polygon
            .extreme_points()
            .iter()
            .map(|id| polygon.get_point(id))
            .collect();

        let rerun_points = rerun::Points3D::new(
            extreme_points
                .into_iter()
                .map(|p| (p.x, p.y, 0.0))
        );

        self.rec.log(
            format!("{}/extreme_points", name),
            &rerun_points
                .with_radii([10.0])
                .with_colors([(252, 207, 3)]),
        ).unwrap();  // TODO don't unwrap
    }
}