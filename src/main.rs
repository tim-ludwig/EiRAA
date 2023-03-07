mod graph;
mod cnf;
mod colored_graph;
mod vars;
pub mod dummies;
use std::collections::HashMap;
use graph::*;
use dummies::EmptyColorizer;

use crate::colored_graph::ColoredGraph;

fn main() {
    use std::fs::File;

    let g =
        graph! {
        0 => 1, 2, 3, 4;
        1 => 5, 6;
        2 => 7, 8;
        3 => 9, 10, 14;
        4 => 11, 13;
        12 => 17;
        14 => 15;
        15 => 16;
        16 => 17;
        17 => 18;
        18 => 19;
        19 => 20;
        20 => 21
    };
    let mut f = File::create("example1.dot").unwrap();
    colored_graph::render_to(
        ColoredGraph::new(g.clone(), EmptyColorizer::edge_coloring, Graph::johnson_vcol),
        &mut f
    );

    let mut c: HashMap<Edge, u32> = HashMap::new();

    c.insert(Edge { u: 0, v: 1 }, 1);
    c.insert(Edge { u: 0, v: 2 }, 2);
    c.insert(Edge { u: 0, v: 3 }, 3);
    c.insert(Edge { u: 0, v: 4 }, 4);

    c.insert(Edge { u: 1, v: 5 }, 3);
    c.insert(Edge { u: 1, v: 6 }, 4);
    c.insert(Edge { u: 2, v: 7 }, 1);
    c.insert(Edge { u: 2, v: 8 }, 4);
    c.insert(Edge { u: 3, v: 9 }, 1);
    c.insert(Edge { u: 3, v: 10 }, 2);
    c.insert(Edge { u: 4, v: 11 }, 1);
    c.insert(Edge { u: 4, v: 12 }, 2);
    //c.insert(Edge{u: 4, v: 13}, 3);

    c.insert(Edge { u: 3, v: 14 }, 5);
    c.insert(Edge { u: 14, v: 15 }, 3);
    c.insert(Edge { u: 15, v: 16 }, 5);
    c.insert(Edge { u: 16, v: 17 }, 3);

    g.vizing_recolor(0, &mut c, 1, 5);

    println!("{:?}", c.get(&(Edge { u: 0, v: 1 })));
    println!("{:?}", c.get(&(Edge { u: 0, v: 2 })));
    println!("{:?}", c.get(&(Edge { u: 0, v: 3 })));
    println!("{:?}", c.get(&(Edge { u: 0, v: 4 })));
    println!("{:?}", c.get(&(Edge { u: 3, v: 14 })));
    println!("{:?}", c.get(&(Edge { u: 14, v: 15 })));
    println!("{:?}", c.get(&(Edge { u: 15, v: 16 })));
    println!("{:?}", c.get(&(Edge { u: 16, v: 17 })));
    println!();

    println!("{:#?}", g.vizing_ecol());

    let g =
        graph! {
        0 => 1;
        1 => 2;
        2 => 3;
        3 => 4;
        4 => 0;
        1 => 4
    };
    let mut f2 = File::create("example2.dot").unwrap();
    colored_graph::render_to(
        ColoredGraph::new(g.clone(), Graph::vizing_ecol, Graph::johnson_vcol),
        &mut f2
    );

    println!(
        "{:#?}",
        g.random_clique(3, |_| 20, &mut rand::thread_rng())
    );

    let mut weights = HashMap::new();

    weights.insert(Edge::new(0, 1), 5);
    weights.insert(Edge::new(1, 2), 4);
    weights.insert(Edge::new(2, 3), 3);
    weights.insert(Edge::new(3, 4), 2);
    weights.insert(Edge::new(4, 0), 1);
    weights.insert(Edge::new(4, 1), 3);

    dbg!(&g.min_spanning_tree(&weights));
}