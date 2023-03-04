use std::collections::HashMap;

use std::io::Write;

use crate::Graph;
use crate::Edge;
type Nd = i32;
type Ed = Edge;

pub struct ColoredGraph {
    pub graph: Graph,
    colorizer: fn(&Graph) -> HashMap<Edge, u32>,
    pub edge_colors: HashMap<crate::Edge, u32>,
}

impl ColoredGraph {
    pub fn new(g: Graph, c: fn(&Graph) -> HashMap<Edge, u32>) -> ColoredGraph {
        ColoredGraph { graph: g, colorizer: c, edge_colors: HashMap::new() }
    }

    fn colorize(&mut self) -> () {
        self.edge_colors = self.graph.vizing_ecol();
    }

    fn clone(&self) -> ColoredGraph {
        ColoredGraph {
            graph: self.graph.clone(),
            colorizer: self.colorizer,
            edge_colors: self.edge_colors.clone(),
        }
    }
}
pub fn render_to<'a, W: Write>(cg: ColoredGraph, output: &mut W) {
    let mut copy = cg.clone();
    copy.colorize();
    dot::render(&copy, output).unwrap()
}
impl<'a> dot::Labeller<'a, Nd, Ed> for ColoredGraph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example2").unwrap()
    }
    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.graph.vertices[*n as usize].to_string().into())
    }
    fn edge_label<'b>(&'b self, _: &Ed) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr("".into())
    }
    fn edge_color(&'a self, e: &Ed) -> Option<dot::LabelText<'a>> {
        Some(
            dot::LabelText::LabelStr(
                crate::vars
                    ::getColor(
                        self.edge_colors
                            .get(e)
                            .unwrap_or_else(|| &55)
                            .to_owned()
                    )
                    .into()
            )
        )
    }
    fn kind(&self) -> dot::Kind {
        dot::Kind::Graph
    }
}
impl<'a> dot::GraphWalk<'a, Nd, Ed> for ColoredGraph {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        self.graph.vertices
            .iter()
            .map(|&n| n)
            .collect()
    }
    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        self.graph.edges
            .iter()
            .map(|&e| e)
            .collect()
    }
    fn source(&self, e: &Ed) -> Nd {
        e.u
    }
    fn target(&self, e: &Ed) -> Nd {
        e.v
    }
}