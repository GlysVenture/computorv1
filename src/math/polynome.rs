use std::fmt::{format, write};

pub struct Polynomial {
	deg: i32,
	monomes: Vec<(f64, i32)>,
}

impl Polynomial {
	pub fn new() -> Self {
		Polynomial {
			deg: 0,
			monomes: vec![(0.0, 0)]
		}
	}
	pub fn add(&mut self, monome: (f64, i32)) {
		self.monomes.push(monome);
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
			if it == self.monomes.last().unwrap() {
				str += &*format!("{}^{}", it.0, it.1);
			}
			else {str += &*format!("{}^{} + ", it.0, it.1);}
		}
		write!(f,"{}", str)
	}
}

//Parse
use std::str::FromStr;
impl FromStr for Polynomial {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
			.split(',')
			.collect();

		let x_fromstr = coords[0].parse::<i32>()?;
		let y_fromstr = coords[1].parse::<i32>()?;

		Ok(Point { x: x_fromstr, y: y_fromstr })
	}
}
