use std::env;

pub struct ExperimentParameters {
	pub point_gen_type: String,
	pub max_vertex_count: u64,
	pub trial_count: u64,
	pub step_size: usize
}

impl ExperimentParameters {
	pub fn fetch() -> Option<ExperimentParameters> {
		let args: Vec<String> = env::args().collect();
		if args.len() != 5 {
			println!("Insufficient number of arguments provided.");
			return None;
		}

		let point_gen_type = &args[1];

		let max_vertex_count = match args[2].parse::<u64>() {
			Ok(val) => val,
			Err(_) => {
				println!("Invalid max vertex count provided.");
				return None;
			}
		};

		let trial_count = match args[3].parse::<u64>() {
			Ok(val) => val,
			Err(_) => {
				println!("Invalid trial count provided.");
				return None;
			}
		};

		let step_size = match args[4].parse::<usize>() {
			Ok(val) => val,
			Err(_) => {
				println!("Invalid step size provided.");
				return None;
			}
		};

		Some(ExperimentParameters {
			point_gen_type: point_gen_type.to_string(),
			max_vertex_count: max_vertex_count,
			trial_count: trial_count,
			step_size: step_size
		})
	}
}