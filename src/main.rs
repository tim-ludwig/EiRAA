mod graph;
mod cnf;

use graph::*;

fn main() {
    let g = graph! {
        0 => 1;
        1 => 2, 3;
        2 => 3, 0;
        3 => 4;
        4 => 0, 2
    };

    let col = g.johnson_vcol();
    let k = col.values().copied().max().unwrap_or(0);

    println!("G={g:?} ist drei-färbbar.");
    println!("Johnson findet die {k}-Färbung col={col:#?}");
    println!();

    let g = johnson_witness(5);

    let col = g.johnson_vcol();
    let k = col.values().copied().max().unwrap_or(0);

    println!("Als kreisfreier Graph ist G={g:?} zwei-färbbar.");
    println!("Johnson findet die {k}-Färbung col={col:#?}")
}
