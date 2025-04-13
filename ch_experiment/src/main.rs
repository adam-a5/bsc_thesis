fn convex_hull(points: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)> {

	let mut upper_hull: Vec<(f64, f64)> = Vec::new();
	let mut lower_hull: Vec<(f64, f64)> = Vec::new();

	// F64 cannot be directly compared with F64 due to 
	// the existance of special values. 
	// TODO: Verify whether total_cmp or partial_cmp should be used here.
	points.sort_by(|a, b| a.0.total_cmp(&b.0));

	upper_hull.push(points[0].clone());
	upper_hull.push(points[1].clone());

	// Go from the 3rd element to the last element.
	for i in 2..points.len() {
		upper_hull.push(points[i].clone());
		loop {
			if upper_hull.len() <= 2 {
				break;
			}

			let first_last = upper_hull[upper_hull.len() - 1];
			let second_last = upper_hull[upper_hull.len() - 2];
			let third_last = upper_hull[upper_hull.len() - 3];
			if makes_right_turn(third_last, second_last, first_last) {
				break;
			}

			upper_hull.remove(upper_hull.len() - 2);
		}
	}

	lower_hull.push(points[points.len() - 1].clone());
	lower_hull.push(points[points.len() - 2].clone());
	// Second last element to first element
	for i in (0..points.len()-1).rev() {
		lower_hull.push(points[i].clone());
		loop {
			if lower_hull.len() <= 2 {
				break;
			}

			let first_last = lower_hull[lower_hull.len() - 1];
			let second_last = lower_hull[lower_hull.len() - 2];
			let third_last = lower_hull[lower_hull.len() - 3];
			if makes_right_turn(third_last, second_last, first_last) {
				break;
			}

			lower_hull.remove(lower_hull.len() - 2);
		}
	}

	lower_hull.remove(lower_hull.len() - 1);
	lower_hull.remove(0);

	let mut final_set: Vec<(f64, f64)> = Vec::new();
	for point in upper_hull.iter() {
		final_set.push(point.clone());
	}
	for point in lower_hull.iter() {
		final_set.push(point.clone());
	}
	final_set
}

fn makes_right_turn(point_1: (f64,f64), point_2: (f64,f64), point_3: (f64,f64)) -> bool {
	/*
	 * The cross product can be used to determine whether a right
	 * turn was made.
	 * Cross product is negative => right turn.
	 */
	let cross_product = (point_2.0 - point_1.0)*(point_3.1-point_1.1) - (point_2.1 - point_1.1)*(point_3.0 - point_1.0);
	cross_product < 0.0
}

fn main() {

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

	let ch = convex_hull(&mut points);

	for ch_point in ch.iter() {
		println!("CH point: {:?}", ch_point);
	}

	println!("Hello World from Convex Hull Experiment.");
}
