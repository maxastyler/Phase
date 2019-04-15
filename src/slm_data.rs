use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contains the data for an individual pattern
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PatternData {
    pub l: i32,
    pub a: f64,
    pub k: (f64, f64),
    pub c: (f64, f64),
    pub phase: f64,
}

/// Contains the important data which is used to create a pattern container
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PatternContainerData {
    pub top_left: (f64, f64),
    pub bottom_right: (f64, f64),
    pub pos: (f64, f64),
    pub scale: (f64, f64),
    pub patterns: HashMap<usize, PatternData>,
}
