mod convex_hull_algorithm;
mod andrews_monotone_chain;
mod multithreaded_experiment;
mod random_point_set;
mod random_rectangle_point_set;
mod random_disk_point_set;

use std::env;

use crate::multithreaded_experiment::MultithreadedExperiment;
use crate::andrews_monotone_chain::AndrewsMonotoneChain;
use crate::random_disk_point_set::RandomDiskPointSet;
use crate::random_rectangle_point_set::RandomRectanglePointSet;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() != 5 {
		println!("Insufficient number of arguments provided.");
		return;
	}

	let point_gen_type = &args[1];

	let max_vertex_count = match args[2].parse::<u64>() {
		Ok(val) => val,
		Err(_) => {
			println!("Invalid max vertex count provided.");
			return;
		}
	};

	let trial_count = match args[2].parse::<u64>() {
		Ok(val) => val,
		Err(_) => {
			println!("Invalid trial count provided.");
			return;
		}
	};

	let step_size = match args[2].parse::<usize>() {
		Ok(val) => val,
		Err(_) => {
			println!("Invalid step size provided.");
			return;
		}
	};

	if point_gen_type == "disc" {
		let exp = MultithreadedExperiment {
			algorithm: AndrewsMonotoneChain,
			random_point_set: RandomDiskPointSet::new(25_000.0)
		};
		exp.run(max_vertex_count, trial_count, step_size);
	} else if point_gen_type == "rectangle" {
		let exp = MultithreadedExperiment {
			algorithm: AndrewsMonotoneChain,
			random_point_set: RandomRectanglePointSet::new(25_000.0, 25_000.0)
		};
		exp.run(max_vertex_count, trial_count, step_size);
	} else {
		println!("Invalid point generator provided.");
		return;
	}
}
