use std::cmp::max;
use std::fmt::{format, write};
use crate::math::polynome::ParsePolyError::*;

#[derive(Debug, Clone)]
pub struct Polynomial {
	deg: i32,
	monomes: Vec<(f64, i32)>,
}

#[derive(Debug)]
pub enum ParsePolyError {
	UnknownToken,
	InvalidDegree,
	SyntaxError,
	FloatError,
	IntegerError
}

//Equation stuff
#[derive(Debug)]
pub enum EquationResult {
	AllReal,
	No,
	Unable
}

impl Polynomial {
	pub fn new() -> Self {
		Polynomial {
			deg: 0,
			monomes: vec![]
		}
	}
	pub fn add(&mut self, monome: (f64, i32)) {
		self.deg = max(self.deg, monome.1);
		self.monomes.push(monome);
	}

	fn substract(&mut self, monome: (f64, i32)) {
		self.deg = max(self.deg, monome.1);
		let tmp = (-monome.0, monome.1);
		self.monomes.push(tmp);
	}

	pub fn cleanup(&mut self) {
		for i in 0..self.monomes.len() {
			if i >= self.monomes.len() { break; }

			let mut sum: (f64, i32) = self.monomes[i];
			for m in self.monomes[i + 1..].iter_mut() {
				if m.1 == sum.1 {
					sum.0 += m.0;
					m.0 = 0.0;
				}
			}
			self.monomes[i] = sum;
		}
		self.monomes.retain(|&m| m.0 != 0.0);
		self.monomes.sort_by(|m, m2| {
			m2.1.cmp(&m.1)
		});
		self.deg = self.monomes.iter()
			.map(|m| m.1).max().unwrap_or(0);
	}

	pub fn degree(&self) -> i32{
		self.deg
	}

	pub fn get_monome(&self, degree: i32) -> (f64, i32) {
		let mut m = (0.0, 0);
		for monome in self.monomes.iter() {
			if monome.1 == degree {
				m.0 += monome.0;
				m.1 += monome.1;
			}
		}
		m
	}

	pub fn solve(&self) -> Result<Vec<f64>, EquationResult> {
		let mut p = self.clone();
		p.cleanup();
		match self {
			Self {deg: 0, ..} => p.solve0(),
			Self {deg: 1, ..} => p.solve1(),
			Self {deg: 2, ..} => p.solve2(),
			_ => Err(EquationResult::Unable)
		}
	}

	fn solve0(&self) -> Result<Vec<f64>, EquationResult> {
		if self.monomes.is_empty() { return Err(EquationResult::AllReal); }
		Err(EquationResult::No)
	}

	fn solve1(&self) -> Result<Vec<f64>, EquationResult> {
		let coeffs = (self.get_monome(0).0,
					  self.get_monome(1).0);
		if coeffs.0 != 0.0 {
			if coeffs.1 != 0.0 {
				return Ok(vec![-coeffs.0/coeffs.1]);
			}
			return Err(EquationResult::No);
		}
		else if coeffs.1 == 0.0 {
			return Err(EquationResult::AllReal);
		}
		Ok(vec![0.0])
	}

	fn solve2(&self) -> Result<Vec<f64>, EquationResult> {
		let coeffs = (self.get_monome(0).0,
					  self.get_monome(1).0,
					  self.get_monome(2).0);
		//todo solve 2 degree
		Err(EquationResult::No)
	}
}

//Display
impl std::fmt::Display for Polynomial {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.monomes.is_empty() { return write!(f, "0"); }
		let mut str = String::from("");
		for it in self.monomes.iter() {
			if it != self.monomes.first().unwrap() || it.0.is_sign_negative(){
				if it.0.is_sign_negative() { str += "- "; }
				if it.0.is_sign_positive() { str += "+ "; }
			}

			if it.1 == 0 { str += &*format!("{}", it.0.abs()) }
			else {
				if it.0.abs() != 1.0 { str += &*format!("{}", it.0.abs()) }
				if it.1 == 1 { str += "X" }
				else { str += &*format!("X^{}", it.1) }
			}

			if it != self.monomes.last().unwrap() {str += " ";}
		}
		write!(f,"{}", str)
	}
}

//Parse
fn parse_monome(s: &str) -> Result<(f64, i32), ParsePolyError> {
	let mut slice = s;
	let cut = slice.find('*').ok_or(SyntaxError)?;
	let cut2 = slice.find('^').ok_or(SyntaxError)?;
	let coeff = s[0..cut].trim().parse::<f64>().or(Err(FloatError))?;
	if !slice[cut + 1..cut2].trim().eq_ignore_ascii_case("X") { return Err(SyntaxError) }
	let deg = s[cut2 + 1..slice.len()].trim().parse::<i32>().or(Err(IntegerError))?;
	if deg < 0 { return Err(InvalidDegree) }
	Ok((coeff, deg))
}

//Trait FromStr for .parse()
impl std::str::FromStr for Polynomial {
	type Err = ParsePolyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.find(|c: char| {
			!matches!(c, 'X' | '*' | '.' | '^' | '+' | '-' | 'x') && !c.is_numeric() && !c.is_whitespace()
		}).is_some() { return  Err(UnknownToken) }

		let mut p = Polynomial::new();
		let mut mut_str = s;
		mut_str = mut_str.trim();
		let mut positive = true;
		if mut_str.starts_with('-'){ positive = false }
		mut_str = mut_str.strip_prefix(['-', '+']).unwrap_or(mut_str);

		loop {
			let mut cut = mut_str.find(['-','+'])
				.unwrap_or(mut_str.len());
			if positive { p.add(parse_monome(&mut_str[0..cut])?); }
			else { p.substract(parse_monome(&mut_str[0..cut])?); }
			if cut == mut_str.len() { break; }
			if mut_str.chars().nth(cut).unwrap_or('+') == '-' { positive = false }
			else { positive = true }
			mut_str = &mut_str[cut + 1..mut_str.len()];
		}
		Ok(p)
	}
}
