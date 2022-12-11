use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::vec::Vec;
use rand::prelude::*;

#[derive(Debug)]
#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Edge {
    pub u: i32,
    pub v: i32,
}

impl Edge {
    pub fn new(u: i32, v: i32) -> Self {
        Self {
            u: min(u, v),
            v: max(u, v),
        }
    }
}

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
                    g.add_edge(Edge::new($from, $to));
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

    pub fn incident_edges(&self, u: i32) -> HashSet<Edge> {
        self.edges.iter()
            .copied()
            .filter(|e| e.u == u || e.v == u)
            .collect()
    }

    pub fn neighbours(&self, u: i32) -> HashSet<i32> {
        self.edges.iter()
            .filter(|e| e.u == u || e.v == u)
            .map(|e| if e.u == u { e.v } else { e.u })
            .collect()
    }

    pub fn is_connected(&self, u: i32, v: i32) -> bool {
        self.edges.contains(&Edge::new(u, v))
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

    fn unused_edge_color(&self, u: i32, c: &HashMap<Edge, u32>) -> u32 {
        let mut i = 1;
        let colors: HashSet<u32> = self.incident_edges(u).iter().copied().map(|e| c.get(&e)).filter(|o| o.is_some()).map(|o| *o.unwrap()).collect();
        loop {
            if !colors.contains(&i) {
                return i;
            }

            i += 1;
        }
    }

    pub fn vizing_recolor(&self, u: i32, c: &mut HashMap<Edge, u32>, alpha: u32, beta: u32) {
        if alpha == beta {
            return;
        }

        let mut colors: Vec<u32> = vec!{};
        let mut vertices: Vec<i32> = vec!{};

        let mut U: HashSet<i32> = self.neighbours(u);
        let mut current_color = alpha;

        while let Some(w) = U.iter().copied().filter(|&w| *c.get(&Edge::new(u, w)).unwrap() == current_color).next() {
            U.remove(&w);

            vertices.push(w);
            colors.push(current_color);

            current_color = self.unused_edge_color(w, c);
        }

        if !colors.contains(&current_color) {
            colors.remove(0);
            colors.push(current_color);

            for (&v, a) in vertices.iter().zip(colors) {
                c.insert(Edge::new(u, v), a);
            }
        } else {
            let j = colors.iter().position(|&x| x == current_color).unwrap();
            let vj = *vertices.get(j).unwrap();

            colors.remove(0);

            for (&v, a) in vertices.iter().zip(colors).take(j) {
                c.insert(Edge::new(u, v), a);
            }

            c.remove(&Edge::new(u, vj));

            let mut prev = u;
            let mut next = vj;
            let mut col = beta;
            while let Some(v) = self.neighbours(next).iter().copied().filter(|&v| c.get(&Edge::new(next, v)) == Some(&col)).next() {
                c.insert(Edge::new(prev, next), col);
                c.remove(&Edge::new(next, v));

                col = if col == beta { current_color } else { beta };
                prev = next;
                next = v;
            }

            c.insert(Edge::new(prev, next), col);
        }
    }

    pub fn vizing_ecol(&self) -> HashMap<Edge, u32> {
        let mut c: HashMap<Edge, u32> = HashMap::new();
        let mut g_new = Graph{vertices: self.vertices.clone(), edges: vec!{}};

        let mut deg: u32 = 0;

        for &e in &self.edges {
            if g_new.degree(e.u) == deg as usize || g_new.degree(e.v) == deg as usize {
                deg += 1;

                c.insert(e, deg + 1);
            } else {
                let alpha = g_new.unused_edge_color(e.v, &c);
                let beta = g_new.unused_edge_color(e.u, &c);

                g_new.vizing_recolor(e.u, &mut c, alpha, beta);
                c.insert(e, alpha);
            }

            g_new.add_edge(e);
        }

        c
    }

    pub fn min_spanning_tree(&self, weights: &HashMap<Edge, u32>) -> Graph {
        let mut g = graph!{};
        if self.vertices.is_empty() {
            return g;
        }

        g.vertices.push(*self.vertices.get(0).unwrap());

        while let Some(e) = self.edges.iter().copied()
                                .filter(|e| g.vertices.contains(&e.u) && !g.vertices.contains(&e.v)
                                    || g.vertices.contains(&e.v) && !g.vertices.contains(&e.u))
                                .min_by_key(|e| weights.get(e).expect("All edges need a weight")) {
            g.add_edge(e);
        }

        g
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
        g.add_edge(Edge::new(e.u + offset, e.v + offset));
    }

    for j in 1..=offset {
        g.add_edge(Edge::new(j, j + offset));
    }

    g
}
