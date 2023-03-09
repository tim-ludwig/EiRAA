use std::collections::HashMap;

use crate::graph::{Edge, Graph};

pub struct Colorizer {}
impl Colorizer {
    pub fn empty_edge_coloring(_g: &Graph) -> HashMap<Edge, u32> {
        HashMap::new()
    }
    pub fn empty_vertex_coloring(_g: &Graph) -> HashMap<i32, u32> {
        HashMap::new()
    }
    pub fn green_edge_coloring(g: &Graph) -> HashMap<Edge, u32> {
        let mut colors = HashMap::new();
        for e in &g.edges {
            colors.insert(*e, 0);
        }
        colors
    }
    pub fn green_vertex_coloring(g: &Graph) -> HashMap<i32, u32> {
        let mut colors = HashMap::new();
        for v in &g.vertices {
            colors.insert(*v, 0);
        }
        colors
    }
}
