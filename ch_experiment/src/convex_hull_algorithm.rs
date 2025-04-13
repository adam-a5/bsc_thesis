pub trait ConvexHullAlgorithm {
	fn convex_hull(&self, points: &mut Vec<(f64, f64)>) -> Vec<(f64, f64)>;
}