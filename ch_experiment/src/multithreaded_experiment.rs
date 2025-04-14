use std::time::Instant;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;
use std::marker::{Send, Sync};

use crate::convex_hull_algorithm::ConvexHullAlgorithm;
use crate::random_point_set::RandomPointSet;

pub struct MultithreadedExperiment<A: ConvexHullAlgorithm + Send + Sync, B: RandomPointSet + Send + Sync> {
	pub algorithm: A,
	pub random_point_set: B,
}

impl<
	A: ConvexHullAlgorithm + Send + Sync + Clone + 'static,
	B: RandomPointSet + Send + Sync + Clone + 'static
> MultithreadedExperiment<A, B> {
	pub fn run(&self, max_vertex_count: u64, trial_count: u64, step_size: usize) {

		let experiment_results = Arc::new(Mutex::new(Vec::<(u64,f64)>::new()));
		let task_queue = Arc::new(Mutex::new(VecDeque::<u64>::new()));

		{
			let mut queue = task_queue.lock().unwrap();
			for i in (3..=max_vertex_count).step_by(step_size) {
				queue.push_back(i);
			}
		}

		let thread_target = 11;
		let mut handles = Vec::new();
		for _ in 0..thread_target {
			let experiment_results = experiment_results.clone();
			let point_generator = self.random_point_set.clone();
			let task_queue = task_queue.clone();
			let algorithm = self.algorithm.clone();
			let handle = thread::spawn(move || {
				loop {
					let current_n = {
						let mut queue = task_queue.lock().unwrap();
						if queue.is_empty() {
							break;
						}
						queue.pop_front().unwrap()
					};

					let mut total_ns: u128 = 0;
					for _ in 0..trial_count {
						let mut points = point_generator.generate(current_n);
						
						let start = Instant::now();
						let _ = algorithm.convex_hull(&mut points);
						let duration = start.elapsed();
						let duration_ns = duration.as_nanos();
						total_ns += duration_ns;
					}
					let average_duration: f64 = total_ns as f64 / trial_count as f64;

					{
						let mut results = experiment_results.lock().unwrap();
						results.push((current_n, average_duration));
					}

					println!("Just completed: {}", current_n);
				}
			});
			handles.push(handle);
		}


		for handle in handles {
			handle.join().unwrap();
		}

		println!("Processing experiment results ...");

		let mut results = experiment_results.lock().unwrap();
		results.sort_by_key(|a| a.0);

		let mut file = OpenOptions::new()
			.create(true)
			.append(true)
			.open("experiment_results.csv")
			.expect("Could not open/create results file.");
		
		let header_line = "input_size,duration\n";
		file.write_all(header_line.as_bytes()).expect("Couldn't write header line.");
		
		for result in results.iter() {
			let line = format!("{},{}\n", result.0, result.1);
			file.write_all(line.as_bytes()).expect("Could not write to results file.");
		}

	}
}