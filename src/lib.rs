pub use rand::prelude::*;
pub use rand_pcg::Pcg64;
pub use rand::seq::SliceRandom;
pub use rand::distributions::Bernoulli;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;



pub struct SimulationConfig {
	pub num_features: u32,
	pub num_traits: u32,
	pub width: u32,
	pub height: u32,
	pub num_iterations: usize,
	pub rng: rand_pcg::Lcg128Xsl64,
}

impl SimulationConfig {
	pub fn new(num_features: u32, num_traits: u32, width: u32, height: u32, num_iterations: usize, rng: rand_pcg::Lcg128Xsl64) -> SimulationConfig {
		SimulationConfig {
			num_features,
			num_traits,
			width,
			height,
			num_iterations,
			rng,

		}
	}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
	x: u32,
	y: u32,
}

impl Point {
	pub fn new(x: u32, y: u32) -> Point {
		Point{ x, y }
	}

	pub fn x(&self) -> usize {
		self.x as usize
	}

	pub fn y(&self) -> usize {
		self.y as usize
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Territory {
	width: u32,
	height: u32,
	pub territory: Vec<Vec<Individual>>,
}

impl Territory {
	pub fn new(config: &mut SimulationConfig) -> Territory {
		let mut territory: Vec<Vec<Individual>> = vec![];

		for x in 0..config.width {
			
			let mut y_vec: Vec<Individual> = vec![];

			for y in 0..config.height {
				let location = Point { x, y };
				y_vec.append(&mut vec![Individual::new(location, config)]);
			}

			territory.append(&mut vec![y_vec]);
		};

		Territory { width: config.width, height: config.height, territory }
	}

	pub fn run_simulation(config: &mut SimulationConfig) -> Territory {
		let mut simulation = Territory::new(config);

		for _ in 0..config.num_iterations {
			// too much fighting with borrow checker, will look ugly but it's the best I can do at this point
			// get coordinates for individual
			let (x, y): (usize, usize) = (config.rng.gen_range(0..config.width) as usize, config.rng.gen_range(0..config.height) as usize);
	
			let chosen = &simulation.territory[x][y];
			let loc_neighbor = chosen.choose_random_neighbor(config);
			let neighbor = simulation.return_neighbor_clone(loc_neighbor);
	
			let chosen = &mut simulation.territory[x][y];
			chosen.interact(neighbor, config);
		}
	
		// let file = File::create("./simulation_terrain.json").unwrap();
		// serde_json::to_writer(&file, &simulation).unwrap();
	
		simulation
	}

	pub fn run_count_cultures(&self, config: &SimulationConfig) -> HashMap<Vec<u32>, u32> {
		let mut culture_hashmap: HashMap<Vec<u32>, u32> = HashMap::new();
	
		for i in 0..config.width {
			for j in 0..config.height {
				let count = culture_hashmap
					.entry(self.territory[i as usize][j as usize].clone_cultural_features())
					.or_insert(0);
				*count += 1;
	
			}
		}
	
		culture_hashmap
	}

	pub fn run_assign_label_to_cultures(&self, config: &SimulationConfig) -> HashMap<Vec<u32>, u32> {
		let mut current_label: u32 = 1;
		let mut map: HashMap<Vec<u32>, u32> = HashMap::new();
		
		for i in 0..config.width {
			for j in 0..config.height {
				let current_features = self.territory[i as usize][j as usize].clone_cultural_features();
				if !map.contains_key(&current_features) {
					map.insert(current_features, current_label);
					current_label += 1;
				}
				/* 
				 * 
				 */
			}
		}

		map
	}

	pub fn run_assign_label_to_individuals(&mut self, config: &SimulationConfig, label_map: HashMap<Vec<u32>, u32>) {
		for i in 0..config.width {
			for j in 0..config.height {
				let current_features = self.territory[i as usize][j as usize].clone_cultural_features();
				let current_label = label_map.get(&current_features).unwrap();
				self.territory[i as usize][j as usize].culture_label = *current_label;
			}
		}	
	}
	
	pub fn calc_prop_pop_in_largest_cluster(config: &SimulationConfig, map: &HashMap<Vec<u32>, u32>) -> f32 {
		*map.values().max().unwrap() as f32 / config.height.pow(2) as f32
	}

	pub fn return_neighbor_clone(&self, location: &Point) -> Individual {
		let neighbor = &self.territory[location.x()][location.y()];

		Individual {
			location: Point{x: location.x() as u32, y: location.y() as u32},
			cultural_features: neighbor.cultural_features.clone(),
			neighbors: vec![],
			culture_label: 0,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishedTerritory {
	width: u32,
	height: u32,
	pub territory: Vec<Vec<Individual>>,
	pub culture_counter: HashMap<Vec<u32>, u32>,
	pub culture_labels: HashMap<Vec<u32>, u32>,
}

impl FinishedTerritory {
	pub fn new(config: &SimulationConfig, simulation: Territory) -> FinishedTerritory {
		let culture_counter = simulation.run_count_cultures(config);
		let culture_labels = HashMap::new();
		let width = simulation.width;
		let height = simulation.height;
		let territory = simulation.territory;
		FinishedTerritory { width, height, territory, culture_counter, culture_labels }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
	location: Point,
	cultural_features: Vec<u32>,
	neighbors: Vec<Point>,
	culture_label: u32,
}

impl Individual {
	pub fn new(location: Point, config: &mut SimulationConfig) -> Individual {

		let cultural_features: Vec<u32> = (0..config.num_features)
			.map(|_| config.rng.gen_range(0..config.num_traits)).collect();

		let h = config.height - 1;
		let w = config.width -1;

		let neighbors: Vec<Point> = match location {
			// corner points
			_ if location.x == 0 && location.y == 0 => vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }],
			_ if location.x == w && location.y == h => vec![Point { x: w, y: h - 1 }, Point { x: w - 1, y: h }],
			_ if location.x == 0 && location.y == h => vec![Point { x: 0, y: h - 1 }, Point { x: 1, y: 9 }],
			_ if location.x == w && location.y == 0 => vec![Point { x: 9, y: 1 }, Point { x: 8, y: 0 }],
	
			// edge points
			_ if location.x == 0 && location.y <  h => vec![
				Point { x: 0, y: location.y - 1 }, Point { x: 0, y: location.y + 1 }, Point { x: 1, y: location.y }
			],
			_ if location.x == w && location.y <  h => vec![
				Point { x: w, y: location.y - 1 }, Point { x: w, y: location.y + 1 }, Point { x: w - 1, y: location.y }
			],
			_ if location.x <  w && location.y == 0 => vec![
				Point { x: location.x - 1, y: 0 }, Point { x: location.x + 1, y: 0 }, Point { x: location.x, y: 1}
			],
			_ if location.x <  w && location.y == h => vec![
				Point { x: location.x - 1, y: h }, Point { x: location.x + 1, y: h }, Point { x: location.x, y: h - 1}
			],
	
			// interior points
			_ if location.x <  w && location.y <  h => vec![
				Point { x: location.x, y: location.y - 1}, Point { x: location.x, y: location.y + 1},
				Point { x: location.x - 1, y: location.y }, Point { x: location.x + 1, y: location.y}
			],
	
			_ => vec![],
		};

		Individual { location, cultural_features, neighbors, culture_label: 0 }
	}

	pub fn choose_random_neighbor(& self, config: &mut SimulationConfig) -> &Point {
		let vs = &self.neighbors;
		vs.choose(&mut config.rng).unwrap()
	}

	pub fn interact(&mut self, other: Individual, config: &mut SimulationConfig) {
		let mine = &self.cultural_features;
		let theirs = &other.cultural_features;

		let denominator = mine.len();

		let numerator = mine.iter()
			.zip(theirs.iter())
			.filter(|&(a, b)| a == b)
			.count();
		
		// IF ALL ELEMENTS ARE THE SAME RETURN
		if numerator == denominator {
			return;
		}
		
		let is_interact = Bernoulli::from_ratio(numerator as u32, denominator as u32)
			.unwrap()
			.sample(&mut config.rng);

		if is_interact {
			// println!("individual {:?} \n\nand neighbor {:?} \n\ninteract: {}\n\n\n", self, other, is_interact);
 			let different_characteristics: Vec<usize> = (0..denominator)
				.zip(mine.iter().zip(theirs.iter()))
				.map(|(i, (m, t))| (i, m != t))
				.filter(|(_, b)| *b)
				.map(|(i, _)| i)
				.collect();
			// println!("Different characteristics: {:?}", different_characteristics);
			
			let feature_to_mutate = *different_characteristics.choose(&mut config.rng).unwrap();
			// println!("Feature to mutate: {:?}\nself value {:?}\nother value {:?}", feature_to_mutate, self.cultural_features[feature_to_mutate], other.cultural_features[feature_to_mutate]);
			self.cultural_features[feature_to_mutate] = other.cultural_features[feature_to_mutate];
			// println!("individual {:?} \n\nand neighbor {:?} \n\n", self, other);

		}
		
	}

	pub fn print_cultural_features(&self) {
		println!("{:?}", self.cultural_features)
	}

	pub fn clone_cultural_features(&self) -> Vec<u32> {
		self.cultural_features.clone()
	}
}

#[cfg(Tests)]
mod test {
	#[test]
	fn name() {
		unimplemented!();
	}
}