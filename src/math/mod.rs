mod polynome;
pub use polynome::*;

enum EquationResult {
	AllReal,
	No
}

fn solve(p: Polynomial) -> Result<Vec<f64>, EquationResult> {
	Err(EquationResult::No)
}
