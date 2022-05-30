mod math;

use crate::math::*;


fn main() -> Result<(), String>{

    let input = std::env::args().nth(1).ok_or("Arguments")?;
    let p = equ_to_poly(input.as_str())?;

    // let p2 = equ_to_poly("1*X^2 = -4 * X^0")?;

    // println!("test: {} = 0", p2);
    // println!("solve: {:?}", p2.solve());
    // println!("input debug: {:?}", p2);
    println!("Equation: {} = 0", p);
    println!("deg: {}", p.degree());
    let result = p.solve();
    if result.is_ok() {
       let solutions = result.unwrap();
        print!("Solutions: {{");
        for s in &solutions {
            print!("{}", s);
            if s != solutions.last().unwrap() { print!(","); }
        }
        println!("}}");
    }
    else {
        match result.unwrap_err() {
            EquationResult::All => { println!("Solution: ℂ") }
            EquationResult::No => { println!("Solution: {{∅}}") }
            EquationResult::Unable => { println!("Unable to find solution, degree too high") }
        }
    }
    Ok(())
}
