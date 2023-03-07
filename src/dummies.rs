use std::collections::HashMap;

use crate::graph::{ Edge, Graph };

pub struct EmptyColorizer {}
impl EmptyColorizer {
    pub fn edge_coloring(_g: &Graph) -> HashMap<Edge, u32> {
        HashMap::new()
    }
    pub fn vertex_coloring(_g: &Graph) -> HashMap<i32, u32> {
        HashMap::new()
    }
}