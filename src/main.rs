use std::fs::File;
use std::io::prelude::*;
pub use rand::prelude::*;
pub use rand_pcg::Pcg64;

use axelrod_model_simulation as ax;

fn main() {
    let seeds: [u64; 200] = [17093, 36562, 20889, 44060, 36378, 55386, 63513, 6699, 16097, 30514, 25163, 27837, 42125, 55577, 59320, 21324, 61220, 57763, 24572, 3258, 17716, 8222, 59069, 61777, 11677, 17176, 62738, 16063, 36766, 54807, 61076, 26670, 61762, 97, 3551, 33086, 32914, 64758, 25232, 26597, 45683, 13783, 50008, 6597, 7472, 9538, 24182, 52475, 58592, 45465, 19739, 12094, 52267, 57829, 36724, 27192, 41722, 14325, 59822, 33665, 35854, 54239, 39959, 41337, 13875, 23129, 45669, 41770, 49843, 53256, 44130, 34648, 13428, 34287, 22918, 50502, 3379, 2845, 11100, 45190, 11463, 26014, 10786, 48470, 14852, 32996, 51181, 57481, 56361, 44070, 1290, 33012, 37210, 45781, 37320, 6228, 26848, 13693, 53179, 19901, 13205, 60437, 33933, 34677, 24856, 59255, 23613, 49835, 5999, 59653, 3045, 30808, 60502, 19174, 18114, 3369, 31911, 12246, 60949, 5703, 64975, 14225, 12387, 46597, 10320, 6065, 30005, 22061, 25933, 28261, 19328, 21017, 16244, 54661, 9469, 894, 60765, 17392, 14931, 33102, 3029, 59344, 4103, 45520, 5732, 12611, 42760, 35212, 31301, 55768, 48363, 25231, 8581, 56658, 31702, 7696, 52334, 36042, 7793, 771, 17900, 20652, 62298, 12763, 32493, 6520, 42201, 47155, 59706, 46996, 27107, 7596, 33468, 15436, 17386, 53160, 24849, 28401, 44390, 40411, 55794, 22001, 63075, 60741, 13003, 10506, 6910, 37544, 1541, 39605, 17056, 60773, 5027, 15807, 60772, 40683, 49042, 2843, 8729, 64037];
    let dif_sizes: [u32; 2] = [10, 20];
    let dif_features: [u32; 3] = [5, 10, 15];
    let dif_traits: [u32; 3] = [5, 10, 15];
    let num_sim_per_combination = 10;

    let mut seed = seeds.iter();
    let mut file = File::create("./results_number_of_clusters_per_combination.csv").unwrap();
    writeln!(file, "Size,Features,Traits,Avg Num of Clusters").unwrap();

    for s in dif_sizes.iter() {
        for f in dif_features.iter() {
            for t in dif_traits.iter() {
                let mut totals: f32 = 0.;
                for c in 0..num_sim_per_combination {
                    // let my_seed = *seed.next().unwrap();
                    // println!("seed: {}", my_seed);
                    let mut config = ax::SimulationConfig::new(
                        *f, *t, *s, *s, Pcg64::seed_from_u64(*seed.next().unwrap())
                    );

                    // totals += ax::run_simulation_and_count(&mut config) as f32;
                    let sim = ax::run_simulation(&mut config);
                    let num_cultures = ax::run_count_cultures(&config, &sim);
                    totals += num_cultures.len() as f32;

                    if (c == num_sim_per_combination - 1) && (*f == 10){
                        let title = format!("./simulation_terrain_features-{}_traits-{}_size-{}.json", *f, *t, *s);
                        let file = File::create(title).unwrap();
                        serde_json::to_writer(&file, &sim).unwrap();
                    }
                }
                let avg: f32 = totals / num_sim_per_combination as f32;
                writeln!(file, "{},{},{},{}", s, f, t, avg).unwrap();
            }
        }
    }
    

}