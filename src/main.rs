use std::fs::File;
pub use rand::prelude::*;
pub use rand_pcg::Pcg64;
use std::collections::HashMap;
use axelrod_model_simulation as ax;

fn _add_random(ran_gen: &mut rand_pcg::Lcg128Xsl64, val: i32) -> i32 {
    let r: i32 = ran_gen.gen_range(1..11);
    println!("{}", r);
    println!("{} + {} = {}", val, r, val + r);
    val + r
}

fn main() {
    let mut config = ax::SimulationConfig::new(5, 15, 20, 20, Pcg64::seed_from_u64(42));

    let mut simulation = ax::Territory::new(&mut config);

    // one round of the simulation
    for _ in 0..1_000_000 {
        // too much fighting with borrow checker, will look ugly but it's the best I can do at this point
        // get coordinates for individual
        let (x, y): (usize, usize) = (config.rng.gen_range(0..config.width) as usize, config.rng.gen_range(0..config.height) as usize);

        let chosen = &simulation.territory[x][y];
        let loc_neighbor = chosen.choose_random_neighbor(&mut config);
        let neighbor = simulation.return_neighbor_clone(loc_neighbor);

        let chosen = &mut simulation.territory[x][y];
        chosen.interact(neighbor, &mut config);
    }
    
    let mut culture_hashmap: HashMap<Vec<u32>, u32> = HashMap::new();

    for i in 0..config.width {
        for j in 0..config.height {
            // simulation.territory[i as usize][j as usize].print_cultural_features();
            let count = culture_hashmap
                .entry(simulation.territory[i as usize][j as usize].clone_cultural_features())
                .or_insert(0);
            *count += 1;

        }
    }

    for (k, v) in culture_hashmap.iter() {
        println!("Cultural features combination {:?} appeared {} times", k, v);
    }

    let file = File::create("./simulation_terrain.json").unwrap();
    serde_json::to_writer(&file, &simulation).unwrap();

}