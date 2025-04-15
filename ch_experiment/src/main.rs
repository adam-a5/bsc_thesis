mod convex_hull_algorithm;
mod andrews_monotone_chain;
mod experiment;
mod multithreaded_experiment;
mod random_point_set;
mod random_rectangle_point_set;
mod random_disk_point_set;

use crate::multithreaded_experiment::MultithreadedExperiment;
use crate::andrews_monotone_chain::AndrewsMonotoneChain;
use crate::random_disk_point_set::RandomDiskPointSet;

fn main() {
	let exp = MultithreadedExperiment {
		algorithm: AndrewsMonotoneChain,
		random_point_set: RandomDiskPointSet::new(25_000.0)
	};
	exp.run(20_000, 1, 100);	
}
