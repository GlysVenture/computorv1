mod math;

use crate::math::*;

fn poly_err(e: ParsePolyError) -> Result<Polynomial, String>{
    match e {
        ParsePolyError::SyntaxError => {Err("Polynomial syntax error".to_owned()) }
        ParsePolyError::IntegerError => {Err("Integer error".to_owned()) }
        ParsePolyError::FloatError => {Err("Float error".to_owned()) }
        ParsePolyError::UnknownToken => {Err("Unknown token".to_owned()) }
        ParsePolyError::InvalidDegree => {Err("Invalid degree".to_owned()) }
    }
}


fn main() -> Result<(), String>{

    // let input = std::env::args().nth(1).ok_or("Arguments")?;
    // let p = equ_to_poly(input.as_str())?;

    let p2 = equ_to_poly("-1*X ^ 2 + 5.5 * X ^ 2 = 2.3 * X^3 + 4.5 * X ^ 2 - 7 * X ^ 4")?;

    println!("test: {}", p2);
    println!("input debug: {:?}", p2);
    // println!("input: {}", p);
    // println!("input debug: {:?}", p);
    Ok(())
}
