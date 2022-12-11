mod graph;
mod cnf;

use std::collections::HashMap;
use graph::*;

fn main() {
    let g = graph! {
        0 => 1, 2, 3, 4;
        1 => 5, 6;
        2 => 7, 8;
        3 => 9, 10, 14;
        4 => 11, 12, 13;
        14 => 15;
        15 => 16;
        16 => 17
    };

    let mut c: HashMap<Edge, u32> = HashMap::new();

    c.insert(Edge{u: 0, v: 1}, 1);
    c.insert(Edge{u: 0, v: 2}, 2);
    c.insert(Edge{u: 0, v: 3}, 3);
    c.insert(Edge{u: 0, v: 4}, 4);

    c.insert(Edge{u: 1, v: 5}, 3);
    c.insert(Edge{u: 1, v: 6}, 4);
    c.insert(Edge{u: 2, v: 7}, 1);
    c.insert(Edge{u: 2, v: 8}, 4);
    c.insert(Edge{u: 3, v: 9}, 1);
    c.insert(Edge{u: 3, v: 10}, 2);
    c.insert(Edge{u: 4, v: 11}, 1);
    c.insert(Edge{u: 4, v: 12}, 2);
    //c.insert(Edge{u: 4, v: 13}, 3);

    c.insert(Edge{u: 3, v: 14}, 5);
    c.insert(Edge{u: 14, v: 15}, 3);
    c.insert(Edge{u: 15, v: 16}, 5);
    c.insert(Edge{u: 16, v: 17}, 3);

    g.vizing_recolor(0, &mut c, 1, 5);

    println!("{:?}", c.get(&Edge{u: 0, v: 1}));
    println!("{:?}", c.get(&Edge{u: 0, v: 2}));
    println!("{:?}", c.get(&Edge{u: 0, v: 3}));
    println!("{:?}", c.get(&Edge{u: 0, v: 4}));
    println!("{:?}", c.get(&Edge{u: 3, v: 14}));
    println!("{:?}", c.get(&Edge{u: 14, v: 15}));
    println!("{:?}", c.get(&Edge{u: 15, v: 16}));
    println!("{:?}", c.get(&Edge{u: 16, v: 17}));
    println!();

    let g = graph!{
        0 => 1;
        1 => 2;
        2 => 3;
        3 => 4;
        4 => 0
    };

    println!("{:?}", g.vizing_ecol());
}
