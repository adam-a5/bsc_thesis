use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;
use std::marker::{Send, Sync};

use uuid::Uuid;

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

		let uuid = Uuid::new_v4().to_string();

		let experiment_results = Arc::new(Mutex::new(Vec::<(u64,f64)>::new()));
		let task_queue = Arc::new(Mutex::new(VecDeque::<u64>::new()));

		{
			let mut queue = task_queue.lock().unwrap();
			for i in (3..=max_vertex_count).step_by(step_size) {
				queue.push_back(i);
			}
		}

		let available = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
		let mut thread_target = available - 1;
		if thread_target == 0 {
			thread_target = 1;
		}
		let mut handles = Vec::new();
		for _ in 0..thread_target {
			let experiment_results = experiment_results.clone();
			let point_generator = self.random_point_set.clone();
			let task_queue = task_queue.clone();
			let algorithm = self.algorithm.clone();
			let handle = thread::spawn(move || {
				Self::worker_thread(
					task_queue,
					point_generator,
					algorithm,
					experiment_results,
					trial_count
				);
			});
			handles.push(handle);
		}

		let uuid_for_reporter = uuid.clone();
		let experiment_results_for_reporter = experiment_results.clone();
		let reporter_handle = thread::spawn(move || {
			Self::reporter_thread(
				max_vertex_count,
				step_size,
				experiment_results_for_reporter,
				uuid_for_reporter
			);
		});
		handles.push(reporter_handle);

		for handle in handles {
			handle.join().unwrap();
		}

		println!("Processing experiment results ...");

		let mut results = experiment_results.lock().unwrap();
		results.sort_by_key(|a| a.0);

		let mut file = OpenOptions::new()
			.create(true)
			.append(true)
			.open(format!("result_{}.csv", uuid))
			.expect("Could not open/create results file.");
		
		let header_line = "input_size,hull_size\n";
		file.write_all(
			header_line.as_bytes()
		).expect("Couldn't write header line.");
		
		for result in results.iter() {
			let line = format!("{},{}\n", result.0, result.1);
			file.write_all(
				line.as_bytes()
			).expect("Could not write to results file.");
		}
	}

	pub fn worker_thread(
		task_queue: Arc<Mutex<VecDeque<u64>>>,
		point_generator: B,
		algorithm: A,
		experiment_results: Arc<Mutex<Vec<(u64,f64)>>>,
		trial_count: u64
	) {
		loop {
			let current_n = {
				let mut queue = task_queue.lock().unwrap();
				if queue.is_empty() {
					break;
				}
				queue.pop_front().unwrap()
			};

			let mut total_size: u64 = 0;

			for _ in 0..trial_count {
				let mut points = point_generator.generate(current_n);
				let hull = algorithm.convex_hull(&mut points);
				total_size += hull.len() as u64;
			}

			let average_hull_size: f64 = total_size as f64 / trial_count as f64;

			{
				let mut results = experiment_results.lock().unwrap();
				results.push((current_n, average_hull_size));
			}

		}
	}

	pub fn reporter_thread(
		max_vertex_count: u64,
		step_size: usize,
		experiment_results: Arc<Mutex<Vec<(u64,f64)>>>,
		uuid: String
	) {
		let expected_results = max_vertex_count / step_size as u64;
		let client = reqwest::blocking::Client::new();
		loop {
			let progress = {
				let results = experiment_results.lock().unwrap();
				if results.len() as u64 >= expected_results {
					break;
				}
				results.len()
			};

			let request_body = format!(
				"experiment_id={}&status={}&total={}",
				uuid,
				progress,
				expected_results
			);
			let _ = client
				.post("http://localhost:25565/")
				.body(request_body)
				.send();
			std::thread::sleep(Duration::from_secs(1));
		}
		println!("Experiment: Now 100% complete.");
	}
}