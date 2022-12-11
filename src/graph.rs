use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use rand::prelude::*;

#[derive(Debug)]
#[derive(Hash)]
pub struct Edge {
    pub u: i32,
    pub v: i32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.u == other.u && self.v == other.v
            || self.u == other.v && self.v == other.u
    }
}

impl Eq for Edge {}

#[derive(Debug)]
pub struct Graph {
    pub vertices: Vec<i32>,
    pub edges: Vec<Edge>,
}

#[macro_export]
macro_rules! graph {
    ( $( $from:expr => $($to:expr),* );* ) => {
        {
            let mut g = Graph::new();
            $(
                $(
                    g.add_edge(Edge{u:$from, v:$to});
                )*
            )*
            g
        }
    };
}

impl Graph {
    pub fn new() -> Graph {
        Graph { vertices: vec!{}, edges: vec!{} }
    }

    pub fn add_vertex(&mut self, v: i32) {
        if !self.vertices.contains(&v) {
            self.vertices.push(v);
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        if !self.edges.contains(&edge) {
            self.add_vertex(edge.u);
            self.add_vertex(edge.v);

            self.edges.push(edge);
        }
    }

    pub fn degree(&self, u: i32) -> usize {
        self.edges.iter()
            .filter(|e| e.u == u || e.v == u)
            .count()
    }

    pub fn neighbours(&self, u: i32) -> HashSet<i32> {
        self.edges.iter()
            .filter(|e| e.u == u || e.v == u)
            .map(|e| if e.u == u { e.v } else { e.u })
            .collect()
    }

    pub fn is_connected(&self, u: i32, v: i32) -> bool {
        self.edges.contains(&Edge { u, v })
    }

    pub fn is_clique(&self, vert: &Vec<i32>) -> bool {
        for i in 0..vert.len() {
            for j in (i + 1)..vert.len() {
                if !self.is_connected(vert[i], vert[j]) {
                    return false;
                }
            }
        }

        true
    }

    pub fn random_clique<R, F>(&self, k: usize, q: F, rng: &mut R) -> Option<Vec<i32>>
        where R: Rng, F: Fn(usize) -> usize {
        let n = self.vertices.len();
        let t = ((n.pow(k as u32) * q(n)) as f32 * 2f32.ln()).ceil() as usize;

        for _ in 0..t {
            let guess: Vec<i32> = self.vertices.choose_multiple(rng, k).copied().collect();

            if self.is_clique(&guess) {
                return Some(guess);
            }
        }

        None
    }

    pub fn greedy_is(&self) -> HashSet<i32> {
        self.greedy_is_on_v(self.vertices.iter().copied().collect())
    }

    fn greedy_is_on_v(&self, mut V: HashSet<i32>) -> HashSet<i32> {
        let mut U: HashSet<i32> = HashSet::new();

        while let Some(u) = V.iter().copied().min_by_key(|&u| self.degree(u)) {
            U.insert(u);
            V.remove(&u);
            V = V.difference(&self.neighbours(u)).copied().collect();
        }

        U
    }

    pub fn johnson_vcol(&self) -> HashMap<i32, i32> {
        let mut c: HashMap<i32, i32> = HashMap::new();
        let mut V: HashSet<i32> = self.vertices.iter().copied().collect();
        let mut t = 1;

        loop {
            let U: HashSet<i32> = self.greedy_is_on_v(V.clone());
            if U.is_empty() { break; }

            V = V.difference(&U).copied().collect();

            for u in U {
                c.insert(u, t);
            }

            t += 1;
        }

        c
    }
}

pub fn johnson_witness(i: i32) -> Graph {
    if i == 1 {
        return graph!{1 => 2};
    }

    let prev = johnson_witness(i - 1);
    let V: Vec<i32> = (1..=2_i32.pow(i as u32)).collect();

    let mut g = Graph {vertices: V, edges: vec!{}};

    let offset = 2_i32.pow(i as u32 - 1);

    for e in prev.edges {
        g.add_edge(Edge{u: e.u + offset, v: e.v + offset});
    }

    for j in 1..=offset {
        g.add_edge(Edge{u: j, v: j + offset});
    }

    g
}
