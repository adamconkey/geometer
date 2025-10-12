use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::vertex::VertexId;

#[derive(Debug)]
pub enum StepParseError {
    IO(std::io::Error),
    Regex(regex::Error),
}

impl From<std::io::Error> for StepParseError {
    fn from(value: std::io::Error) -> Self {
        StepParseError::IO(value)
    }
}

impl From<regex::Error> for StepParseError {
    fn from(value: regex::Error) -> Self {
        StepParseError::Regex(value)
    }
}

pub fn parse_steps_from_logs<T>(path: PathBuf) -> Result<Vec<T>, StepParseError>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"DEBUG \[.*\] \w+ (?<data>.*)")?;

    let mut steps = Vec::<T>::new();
    for line in reader.lines() {
        if let Some(caps) = re.captures(&line?) {
            if let Ok(step) = serde_hjson::from_str::<T>(&caps["data"].to_string()) {
                steps.push(step);
            }
        }
    }

    Ok(steps)
}

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

impl GrahamScanStep {
    pub fn hull_tail(&self, num_elements: usize) -> Vec<VertexId> {
        self.hull_ids[self.hull_ids.len() - num_elements..].to_vec()
    }

    pub fn hull_top(&self) -> VertexId {
        self.hull_ids[self.hull_ids.len() - 1]
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
