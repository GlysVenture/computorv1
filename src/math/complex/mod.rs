
#[derive(Debug, PartialEq)]
pub struct Complex {
	real: f64,
	imaginary: f64
}

impl Complex {
	pub fn new(real: f64, imaginary: f64) -> Self {
		Self {
			real,
			imaginary
		}
	}
}

impl std::fmt::Display for Complex {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self { real: 0.0, imaginary: 0.0 } => write!(f,"0"),
			Self { imaginary: 0.0, .. } => write!(f,"{}", self.real),
			Self { real: 0.0, .. } => write!(f,"{}i", self.imaginary),
			&_ => {
				if self.imaginary.is_sign_positive() { write!(f, "{} + {}i", self.real, self.imaginary) }
				else { write!(f, "{} - {}i", self.real, self.imaginary.abs()) }
			}
		}
	}
}
