use crate::convex_hull_algorithm::ConvexHullAlgorithm;
use crate::random_point_set::RandomPointSet;

pub struct Experiment<A: ConvexHullAlgorithm, B: RandomPointSet> {
	pub algorithm: A,
	pub random_point_set: B
}

impl<A: ConvexHullAlgorithm, B: RandomPointSet> Experiment<A, B> {
	pub fn run(&self) {

		for i in 3..1_000 {
			let mut points = self.random_point_set.generate(i);
			let hull = self.algorithm.convex_hull(&mut points);
			println!("Completed experiment for n = {} with hull_c = {}", i, hull.len());
		}

	}
}