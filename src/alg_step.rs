use serde::Deserialize;
use std::fmt;

use crate::vertex::VertexId;

#[derive(Debug, Default, Deserialize)]
pub struct GrahamScanStep {
    pub idx: usize,
    pub new_id: Option<VertexId>,
    pub hull_ids: Vec<VertexId>,
}

impl fmt::Display for GrahamScanStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GrahamScanStep {{ idx: {}", self.idx)?;
        if let Some(new_id) = self.new_id {
            write!(f, ", new_id: {new_id}")?;
        }
        write!(f, ", hull_ids: {:?} }}", self.hull_ids)?;
        Ok(())
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct IncrementalStep {
    pub idx: usize,
    pub new_id: Option<VertexId>,
    pub ut_id: Option<VertexId>,
    pub lt_id: Option<VertexId>,
    pub hull_ids: Vec<VertexId>,
}

impl fmt::Display for IncrementalStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IncrementalStep {{ idx: {}", self.idx)?;
        if let Some(new_id) = self.new_id {
            write!(f, ", new_id: {new_id}")?;
        }
        if let Some(ut_id) = self.ut_id {
            write!(f, ", ut_id: {ut_id}")?;
        }
        if let Some(lt_id) = self.lt_id {
            write!(f, ", lt_id: {lt_id}")?;
        }
        write!(f, ", hull_ids: {:?} }}", self.hull_ids)?;
        Ok(())
    }
}
