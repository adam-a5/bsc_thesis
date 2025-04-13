mod convex_hull_algorithm;
mod andrews_monotone_chain;
mod experiment;
mod random_point_set;
mod random_rectangle_point_set;

use crate::experiment::Experiment;
use crate::andrews_monotone_chain::AndrewsMonotoneChain;
use crate::random_rectangle_point_set::RandomRectanglePointSet;

fn main() {
	let rrps = RandomRectanglePointSet::new(25_000.0, 25_000.0);
	let exp = Experiment {
		algorithm: AndrewsMonotoneChain,
		random_point_set: rrps
	};
	exp.run();	
}
