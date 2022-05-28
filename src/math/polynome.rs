use std::fmt::{format, write};
use crate::math::polynome::ParsePolyError::*;

#[derive(Debug)]
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

impl Polynomial {
	pub fn new() -> Self {
		Polynomial {
			deg: 0,
			monomes: vec![]
		}
	}
	pub fn add(&mut self, monome: (f64, i32)) {
		self.monomes.push(monome);
	}

	fn substract(&mut self, monome: (f64, i32)) {
		let tmp = (-monome.0, monome.1);
		self.monomes.push(tmp);
	}

	pub fn cleanup(&mut self) {

	}

	pub fn degree(&self) -> i32{
		self.deg
	}
}

//Display
impl std::fmt::Display for Polynomial {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut str = String::from("");
		for it in self.monomes.iter() {
			if it != self.monomes.first().unwrap() || it.0.is_sign_negative(){
				if it.0.is_sign_negative() { str += "- "; }
				if it.0.is_sign_positive() { str += "+ "; }
			}
			if it.0.abs() == 1.0 { str += &*format!("X^{}", it.1) }
			else { str += &*format!("{}X^{}", it.0.abs(), it.1) }
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

impl std::str::FromStr for Polynomial {
	type Err = ParsePolyError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.find(|c: char| {
			!matches!(c, 'X' | '*' | '.' | '^' | '+' | '-') && !c.is_numeric() && !c.is_whitespace()
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
