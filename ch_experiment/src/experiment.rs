use crate::convex_hull_algorithm::ConvexHullAlgorithm;

pub struct Experiment<A: ConvexHullAlgorithm> {
	pub algorithm: A
}

impl<A: ConvexHullAlgorithm> Experiment<A> {
	pub fn run(&self) {
		let mut points: Vec<(f64,f64)> = vec![
			(1.0,1.0),
			(3.0,0.0),
			(4.0,3.0),
			(3.0,5.0),
			(0.0,4.0),
			(6.0,4.0),
			(2.0,2.0),
			(3.0,1.0)
		];

		let hull = self.algorithm.convex_hull(&mut points);
		for p in hull.iter() {
			println!("CH point: {:?}", p);
		}
	}
}