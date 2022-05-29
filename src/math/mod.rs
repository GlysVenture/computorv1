mod polynome;

pub use polynome::*;


//Warning, might not like situations like " ...4 + -2" where will not swap signs todo?
pub fn equ_to_poly(s: &str) -> Result<Polynomial, String> {
	let cut = s.find('=').ok_or("Equation misses = sign")?;

	let mut poly = s[..cut].parse::<Polynomial>()
		.or_else(|e| Err(format!("{:?}", e)))?;
	let poly_right = s[cut + 1..].parse::<Polynomial>()
		.or_else(|e| Err(format!("{:?}", e)))?;
	poly = poly - poly_right;
	poly.cleanup();
	Ok(poly)
}
