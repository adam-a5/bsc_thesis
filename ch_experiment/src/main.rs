mod convex_hull_algorithm;
mod andrews_monotone_chain;
mod multithreaded_experiment;
mod random_point_set;
mod random_rectangle_point_set;
mod random_disk_point_set;
mod experiment_parameters;

use crate::multithreaded_experiment::MultithreadedExperiment;
use crate::andrews_monotone_chain::AndrewsMonotoneChain;
use crate::random_disk_point_set::RandomDiskPointSet;
use crate::random_rectangle_point_set::RandomRectanglePointSet;
use crate::experiment_parameters::ExperimentParameters;

fn main() {
	let exp_params = match ExperimentParameters::fetch() {
		Some(e) => e,
		None => {
			return;
		}
	};

	if exp_params.point_gen_type.as_str() == "disk" {
		let exp = MultithreadedExperiment {
			algorithm: AndrewsMonotoneChain,
			random_point_set: RandomDiskPointSet::new(25_000.0)
		};
		exp.run(
			exp_params.max_vertex_count,
			exp_params.trial_count,
			exp_params.step_size
		);
	} else if exp_params.point_gen_type.as_str() == "rectangle" {
		let exp = MultithreadedExperiment {
			algorithm: AndrewsMonotoneChain,
			random_point_set: RandomRectanglePointSet::new(25_000.0, 25_000.0)
		};
		exp.run(
			exp_params.max_vertex_count,
			exp_params.trial_count,
			exp_params.step_size
		);
	} else {
		println!("Invalid point generator provided.");
		return;
	}
}
