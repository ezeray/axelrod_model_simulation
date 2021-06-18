pub use rand::prelude::*;
pub use rand_pcg::Pcg64;
use axelrod_model_simulation as ax;

fn add_random(ran_gen: &mut rand_pcg::Lcg128Xsl64, val: i32) -> i32 {
    let r: i32 = ran_gen.gen_range(1..11);
    println!("{}", r);
    println!("{} + {} = {}", val, r, val + r);
    val + r
}

fn main() {
    let config = ax::SimulationConfig::new(5, 10, 10, 10);

    let mut simulation = ax::Territory::new(&config);
    let mut rng = Pcg64::seed_from_u64(50);
    
    for i in 0..10 {
        add_random(&mut rng, i as i32);
    }

    // one round of the simulation
    for _ in 0..0 {
        // too much fighting with borrow checker, will look ugly but it's the best I can do at this point
        // get coordinates for individual
        let (x, y): (usize, usize) = (rng.gen_range(0..config.width) as usize, rng.gen_range(0..config.height) as usize);

        let chosen = &simulation.territory[x][y];
        let loc_neighbor = chosen.choose_random_neighbor();
        let neighbor = simulation.return_neighbor_clone(loc_neighbor);

        let chosen = &mut simulation.territory[x][y];
        chosen.interact(neighbor);
    }

}