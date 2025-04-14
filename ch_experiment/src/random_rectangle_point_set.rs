use crate::random_point_set::RandomPointSet;
use rand::Rng;

#[derive(Clone)]
pub struct RandomRectanglePointSet {
	width: f64,
	height: f64
}

impl RandomRectanglePointSet {
	pub fn new(width: f64, height: f64) -> RandomRectanglePointSet {
		RandomRectanglePointSet {
			width: width,
			height: height
		}
	}
}

impl RandomPointSet for RandomRectanglePointSet {
	fn generate(&self, count: u64) -> Vec<(f64,f64)> {
		let mut generator = rand::rng();
		let mut results: Vec<(f64,f64)> = Vec::new();
		for _ in 0..count {
			let random_x: f64 = generator.random();
			let random_y: f64 = generator.random();
			let point: (f64, f64) = (self.width * random_x, self.height * random_y);
			results.push(point);
		}
		results
	}
}