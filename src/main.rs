#![allow(unused)]

use std::fmt;

use rand_distr::{Bernoulli, Poisson, Distribution};
use rand::{self, distributions};

enum Dist {
    Bernoulli,
    Poisson,
}

#[derive(Debug)]
struct GenerationX {
    id: i32,
    parents: Vec<Indivisual>,
}

impl fmt::Display for GenerationX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "===== GENERATION {} =====\n", self.id);
        write!(f, "number of individuals: {}\n", self.parents.len());
        for indivisual in self.parents.iter() {
            write!(f, "My id is {}  --> I have {} children\n", indivisual.id, indivisual.children_len);

        }
        write!(f, "========================\n")
    }
}

#[derive(Debug)]
struct Indivisual {
    gen_id: i32,
    parents_id: i32,
    id: i32,
    children_len: i32,
}

impl Indivisual {
    fn new(dist: &Dist, param: f64, gen_id: i32, parents_id: i32, id: i32) -> Self {
        match dist {
            Dist::Bernoulli => {
                let d = Bernoulli::new(param).unwrap();
                let children_len = if d.sample(&mut rand::thread_rng()) {1} else {0};
                Self {gen_id, parents_id, id, children_len}
            },
            Dist::Poisson => {
                let poi = Poisson::new(param).unwrap();
                let children_len = poi.sample(&mut rand::thread_rng()) as i32;
                Self {gen_id, parents_id, id, children_len}
            },
        }
    }
}

fn main() {
    let distribution: Dist = Dist::Bernoulli;
    let dist_param: f64 = 0.99;
    //let distribution: Dist = Dist::Poisson;
    //let dist_param: f64 = 1.5;

    let x0y = Indivisual::new(&distribution, dist_param, 0, 0, 0);
    let x0y_vec = vec![x0y];
    let x0 = GenerationX { id: 0, parents: x0y_vec};

    let mut x: Vec<GenerationX> = vec![x0];

    let mut gen_id = 0;
    loop {
        let mut id = 0;
        let mut parents_id = 0;
        let mut next_parents: Vec<Indivisual> = vec![];
        if x[gen_id].parents.len() == 0 {
            println!("<<<<<< Everybody DEAD >>>>>>\n");
            break;
        }
        for i in 0..x[gen_id].parents.len() {
            for _ in 0..x[gen_id].parents[i].children_len {
                let borned = Indivisual::new(&distribution, dist_param, gen_id as i32 + 1, parents_id, id);
                next_parents.push(borned);
                id += 1;
            }
            parents_id += 1;
            
        }
        let generation = GenerationX {id: gen_id as i32 + 1, parents: next_parents};
        x.push(generation);
        println!("{}", x[gen_id]);
        gen_id += 1;
    }

}
