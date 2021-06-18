pub use rand::prelude::*;
pub use rand_pcg::Pcg64;
pub use rand::seq::SliceRandom;
pub use rand::distributions::Bernoulli;
use serde::{Serialize, Deserialize};


pub struct SimulationConfig {
	pub num_features: u32,
	pub num_traits: u32,
	pub width: u32,
	pub height: u32,
	pub rng: rand_pcg::Lcg128Xsl64,
}

impl SimulationConfig {
	pub fn new(num_features: u32, num_traits: u32, width: u32, height: u32, rng: rand_pcg::Lcg128Xsl64) -> SimulationConfig {
		SimulationConfig {
			num_features,
			num_traits,
			width,
			height,
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
	pub territory: Vec<Vec<Individual>>
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

	pub fn return_neighbor_clone(&self, location: &Point) -> Individual {
		let neighbor = &self.territory[location.x()][location.y()];

		Individual {
			location: Point{x: location.x() as u32, y: location.y() as u32},
			cultural_features: neighbor.cultural_features.clone(),
			neighbors: vec![],
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Individual {
	location: Point,
	cultural_features: Vec<u32>,
	neighbors: Vec<Point>,
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

		Individual { location, cultural_features, neighbors }
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
}

#[cfg(Tests)]
mod test {
	#[test]
	fn name() {
		unimplemented!();
	}
}