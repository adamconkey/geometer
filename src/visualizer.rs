use crate::util::load_polygon;


pub struct RerunVisualizer {
    rec: rerun::RecordingStream,
}


impl RerunVisualizer {
    pub fn new(name: String) -> Self {
        // TODO don't unwrap
        let rec = rerun::RecordingStreamBuilder::new(name).connect_tcp().unwrap();
        RerunVisualizer { rec }
    }

    // TODO need to have this return Result and handle errors gracefully
    pub fn visualize_triangulation(&self, polygon_name: String, folder: String) {
        // TODO have load function return Result
        let polygon = load_polygon(&polygon_name, &folder);
        let triangulation = polygon.triangulation();

        let rerun_meshes = triangulation.to_rerun_meshes();

        self.rec.log(
            format!("{}/{}/vertices", folder, polygon_name),
            &polygon.to_rerun_points()
                .with_radii([0.3]),
        ).unwrap();  // TODO don't unwrap

        self.rec.log(
            format!("{}/{}/edges", folder, polygon_name),
            &polygon.to_rerun_edges()
        ).unwrap();  // TODO don't unwrap

        for (i, mesh) in rerun_meshes.iter().enumerate() {
            self.rec.log(
                format!("{}/{}/triangle_{}", folder, polygon_name, i),
                mesh
            ).unwrap();  // TODO don't unwrap
        }
    }
}