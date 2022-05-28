mod math;

use std::io;
use crate::math::*;

fn check_err(e: ParsePolyError) -> Result<Polynomial, String>{
    match e {
        ParsePolyError::SyntaxError => {Err("Polynomial syntax error".to_owned()) }
        ParsePolyError::IntegerError => {Err("Integer error".to_owned()) }
        ParsePolyError::FloatError => {Err("Float error".to_owned()) }
        ParsePolyError::UnknownToken => {Err("Unknown token".to_owned()) }
        ParsePolyError::InvalidDegree => {Err("Invalid degree".to_owned()) }
    }
}


fn main() -> Result<(), String>{

    let p = std::env::args().nth(1).ok_or("Arguments")?.parse::<Polynomial>().or_else(check_err)?;

    let p2 = "3*X^4 + 1*X^2 - 5.4 * X^ 23".parse::<Polynomial>().or_else(check_err)?;

    println!("test: {}", p2);
    println!("input: {}", p);
    println!("input debug: {:?}", p);
    Ok(())
}
