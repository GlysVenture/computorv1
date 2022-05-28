mod polynome;

pub use polynome::*;


//Warning, might not like situations like " ...4 + -2" where will not swap signs todo?
pub fn equ_to_poly(str: &str) -> Result<Polynomial, String> {
	let mut s: String = str.to_owned();
	let cut = s.find('=').ok_or("Equation misses = sign")?;
	s = s.replace('=', "-");
	let out = s[cut + 1..s.len()]
		.replace('-', "@")
		.replace('+', "-")
		.replace('@', "+");
	s = s[..cut + 1].to_owned() + out.as_str();


	let mut poly = s.parse::<Polynomial>()
		.or_else(|e| Err(format!("{:?}", e)))?;
	poly.cleanup();
	Ok(poly)
}
