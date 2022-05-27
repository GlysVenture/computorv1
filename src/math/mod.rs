mod polynome;
pub use polynome::Polynomial;

enum EquationResult {
	AllReal,
	No
}

fn solve(p: Polynomial) -> Result<Vec<f64>, EquationResult> {
	Err(EquationResult::No)
}
