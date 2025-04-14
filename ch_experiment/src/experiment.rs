use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;

use crate::convex_hull_algorithm::ConvexHullAlgorithm;
use crate::random_point_set::RandomPointSet;

pub struct Experiment<A: ConvexHullAlgorithm, B: RandomPointSet> {
	pub algorithm: A,
	pub random_point_set: B
}

impl<A: ConvexHullAlgorithm, B: RandomPointSet> Experiment<A, B> {
	pub fn run(&self, max_vertex_count: u64, trial_count: u64, step_size: usize) {

		let mut experiment_results: Vec<(u64, f64)> = Vec::new();

		for i in (3..=max_vertex_count).step_by(step_size) {

			let mut total_ns: u128 = 0;

			for _ in 0..trial_count {
				let mut points = self.random_point_set.generate(i);
				let start = Instant::now();
				let _ = self.algorithm.convex_hull(&mut points);
				let duration = start.elapsed();
				let duration_ns = duration.as_nanos();
				total_ns += duration_ns;
			}

			let average_duration: f64 = total_ns as f64 / trial_count as f64;

			experiment_results.push((i, average_duration));

			println!("Completed {} trials of {} verticies with an average of {}", trial_count, i, average_duration);

		}

		let mut file = OpenOptions::new()
			.create(true)
			.append(true)
			.open("experiment_results.csv")
			.expect("Could not open/create results file.");
		
		let header_line = "input_size,duration\n";
		file.write_all(header_line.as_bytes()).expect("Couldn't write header line.");
		
		for result in experiment_results.iter() {
			let line = format!("{},{}\n", result.0, result.1);
			file.write_all(line.as_bytes()).expect("Could not write to results file.");
		}

	}
}