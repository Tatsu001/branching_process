#![allow(unused)]

use std::process::Child;

use rand_distr::{Normal, Bernoulli, Poisson, Distribution, Pareto};
use rand;

enum Dist {
    Bernoulli,
    Poisson,
}

#[derive(Debug)]
struct GenerationX {
    id: i32,
    parents: Vec<Indivisual>,
}

#[derive(Debug)]
struct Indivisual {
    gen_id: i32,
    parents_id: i32,
    id: i32,
    children_len: i32,
}

impl Indivisual {
    fn new(dist: Dist, param: f64, gen_id: i32, parents_id: i32, id: i32) -> Self {
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

    let x0y = Indivisual::new(Dist::Poisson, 0.5, 0, 0, 0);
    let x0y_vec = vec![x0y];
    let x0 = GenerationX { id: 0, parents: x0y_vec};

    let mut x: Vec<GenerationX> = vec![x0];

    let mut gen_id = 0;
    println!("{:?}", x[0].parents[0].children_len);
    loop {
        let mut id = 0;
        let mut parents_id = 0;
        let mut next_parents: Vec<Indivisual> = vec![];
        if x[gen_id].parents.len() == 0 {
            println!("Everybody DEAD");
            break;
        }
        for i in 0..x[gen_id].parents.len() {
            for _ in 0..x[gen_id].parents[i].children_len {
                let borned = Indivisual::new(Dist::Poisson, 0.5, gen_id as i32, parents_id, id);
                next_parents.push(borned);
                id += 1;
            }
            parents_id += 1;
            
        }
        let generation = GenerationX {id: gen_id as i32, parents: next_parents};
        x.push(generation);
        gen_id += 1;

    }

}
