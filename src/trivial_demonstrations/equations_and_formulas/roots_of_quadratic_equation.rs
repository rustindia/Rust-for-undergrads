use std::io;

/* Prints the roots of a quadratic equation with given coefficients,
 * calculated using quadratic formula.
 */

fn main() {
    println!("Quadratic Equation: ax^2 + bx + c = 0");

    println!("Enter Value of a:");
    let a = read_int();

    println!("Enter Value of b:");
    let b = read_int();

    println!("Enter Value of c:");
    let c = read_int();

    let d = b*b-4*a*c;
    if d<0 {
        println!("No real roots exist");
    } else {
        let root_d = (d as f64).sqrt() as i32;
        let r1 = (-b + root_d)/(2*a);
        let r2 = (-b - root_d)/(2*a);

        println!("Roots are {} and {}", r1,r2)
    }
}

fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error Reading Number");

    num.trim()
        .parse::<i32>()
        .unwrap()
}
