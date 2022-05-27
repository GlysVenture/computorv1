mod math;

use crate::math::Polynomial;


fn main() {
    println!("Hello, world!");
    let mut p = Polynomial::new();

    p.add((4.3, 1));
    p.add((-34.2, 3));

    println!("{}", p);
}
