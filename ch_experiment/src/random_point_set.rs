pub trait RandomPointSet {
	fn generate(&self, count: u64) -> Vec<(f64,f64)>;
}