use crate::Polynomial;

//Equation stuff
#[derive(Debug)]
pub enum EquationResult {
	AllReal,
	No,
	Unable
}

//Solving funcs
impl Polynomial {
	pub fn solve(&self) -> Result<Vec<f64>, EquationResult> {
		let mut p = self.clone();
		p.cleanup();
		if p.monomes.is_empty() { return Err(EquationResult::AllReal); }
		match self {
			Self {deg: 0, ..} => p.solve0(),
			Self {deg: 1, ..} => p.solve1(),
			Self {deg: 2, ..} => p.solve2(),
			_ => Err(EquationResult::Unable)
		}
	}

	fn solve0(&self) -> Result<Vec<f64>, EquationResult> {
		Err(EquationResult::No)
	}

	fn solve1(&self) -> Result<Vec<f64>, EquationResult> {
		let coeffs = (self.get_monome(0).0,
					  self.get_monome(1).0);
		Ok(vec![-coeffs.0/coeffs.1])
	}

	fn solve2(&self) -> Result<Vec<f64>, EquationResult> {
		let coeffs = (self.get_monome(0).0,
					  self.get_monome(1).0,
					  self.get_monome(2).0);
		let disc = (coeffs.1 * coeffs.1) - 4.0 * coeffs.2 * coeffs.0;
		if disc < 0.0 { return Err(EquationResult::No); }
		else if disc == 0.0 { return Ok(vec![-coeffs.1/(2.0 * coeffs.2)]) }
		else { return Ok(vec![
			(-coeffs.1 + disc.sqrt())/(2.0 * coeffs.2),
			(-coeffs.1 - disc.sqrt())/(2.0 * coeffs.2)
		]) }
	}
}