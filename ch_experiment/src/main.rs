mod convex_hull_algorithm;
mod andrews_monotone_chain;
mod experiment;

use crate::experiment::Experiment;
use crate::andrews_monotone_chain::AndrewsMonotoneChain;

fn main() {
	let exp = Experiment { algorithm: AndrewsMonotoneChain };
	exp.run();	
}
