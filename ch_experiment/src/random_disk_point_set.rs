use crate::random_point_set::RandomPointSet;
use rand::Rng;
use std::f64::consts::PI;

#[derive(Clone)]
pub struct RandomDiskPointSet {
	radius: f64
}

impl RandomDiskPointSet {
	pub fn new(radius: f64) -> Self {
		Self {
			radius: radius
		}
	}
}

impl RandomPointSet for RandomDiskPointSet {
	fn generate(&self, count: u64) -> Vec<(f64,f64)> {
		let mut generator = rand::rng();
		let mut results: Vec<(f64,f64)> = Vec::new();
		for _ in 0..count {
			let theta = generator.random_range(0.0..2.0*PI);
			let radius = self.radius * generator.random_range(0.0..1.0);
			let x = radius * theta.cos();
			let y = radius * theta.sin();
			results.push((x,y));
		}
		results
	}
}