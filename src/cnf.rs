use std::f32::consts::PI;
use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct CNF {
    pub clauses: Vec<Vec<i32>>,
}

impl CNF {
    fn eval_clause(c: &Vec<i32>, inter: &Vec<i32>) -> bool {
        c.iter()
            .map(|l| inter.contains(l))
            .reduce(|a, b| a || b)
            .unwrap_or(false)
    }

    pub fn eval(&self, inter: &Vec<i32>) -> bool {
        self.clauses.iter()
            .map(|c| CNF::eval_clause(c, inter))
            .reduce(|a, b| a && b)
            .unwrap_or(true)
    }

    fn var_count(&self) -> i32 {
        self.clauses
            .iter()
            .flat_map(|c| c.iter())
            .map(|l| l.abs())
            .max()
            .unwrap_or(0)
    }

    pub fn random_interpretation<R: Rng>(&self, rng: &mut R) -> Vec<i32> {
        (1..=self.var_count()).map(|l| {
            if rng.gen_bool(0.5) {
                l
            } else {
                -l
            }
        }).collect()
    }

    pub fn random_3sat<R, F>(&self, q: F, rng: &mut R) -> Option<Vec<i32>>
    where R: Rng, F: Fn(i32) -> i32 {
        let n = self.var_count();
        let t = (2.0 * (3.0 * PI * n as f32).sqrt() * (4f32 / 3.0).powi(n)).ceil() as i32  * q(n);

        for _ in 0..t {
            let mut inter = self.random_interpretation(rng);

            for _ in 0..3 * n {
                let unsatisfied_clauses: Vec<&Vec<i32>> = self.clauses.iter()
                    .filter(|&c| {
                        !CNF::eval_clause(c, &inter)
                    }).collect();

                if unsatisfied_clauses.is_empty() {
                    return Some(inter);
                }

                let l = unsatisfied_clauses
                    .choose(rng).unwrap() // choose unsatisfied clause
                    .choose(rng).unwrap() // choose a literal
                    .abs() - 1;           // convert to variable index

                inter[l as usize] *= -1;  // flip interpretation of selected variable, satisfying the selected clause
            }
        }

        None
    }
}

#[macro_export]
macro_rules! cnf {
    ( $( ($( $l:expr ),*) ),* ) => {
        CNF {
            clauses: vec!{$( vec!{ $($l),* } ),*},
        }
    };
}
