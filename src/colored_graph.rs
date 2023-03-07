use std::collections::HashMap;

use std::io::Write;

use crate::Graph;
use crate::Edge;
type Nd = i32;
type Ed = Edge;

pub struct ColoredGraph {
    pub graph: Graph,
    edge_colorizer: fn(&Graph) -> HashMap<Edge, u32>,
    vertex_colorizer: fn(&Graph) -> HashMap<i32, u32>,
    pub edge_colors: HashMap<crate::Edge, u32>,
    pub vertex_colors: HashMap<i32, u32>,
}

impl ColoredGraph {
    pub fn new(
        g: Graph,
        c: fn(&Graph) -> HashMap<Edge, u32>,
        v: fn(&Graph) -> HashMap<i32, u32>
    ) -> ColoredGraph {
        ColoredGraph {
            graph: g,
            edge_colorizer: c,
            vertex_colorizer: v,
            edge_colors: HashMap::new(),
            vertex_colors: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    fn clone(&self) -> ColoredGraph {
        ColoredGraph {
            graph: self.graph.clone(),
            edge_colorizer: self.edge_colorizer,
            vertex_colorizer: self.vertex_colorizer,
            vertex_colors: self.vertex_colors.clone(),
            edge_colors: self.edge_colors.clone(),
        }
    }
}
pub fn render_to<'a, W: Write>(mut cg: ColoredGraph, output: &mut W) {
    cg.edge_colors = (cg.edge_colorizer)(&cg.graph);
    cg.vertex_colors = (cg.vertex_colorizer)(&cg.graph);
    dot::render(&cg, output).unwrap()
}
impl<'a> dot::Labeller<'a, Nd, Ed> for ColoredGraph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("example2").unwrap()
    }
    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n)).unwrap()
    }
    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(format!("{}", n).into())
    }
    fn node_color(&'a self, node: &Nd) -> Option<dot::LabelText<'a>> {
        Some(
            dot::LabelText::LabelStr(
                crate::vars
                    ::getColor(
                        self.vertex_colors
                            .get(node)
                            .unwrap_or_else(|| &55)
                            .to_owned()
                    )
                    .into()
            )
        )
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