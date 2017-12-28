use std::io;

/* Reads two numbers and an arithmetic oeprator.
 * Performs the arithmetic operation on the entered
 * numbers and prints the result
 */

fn main() {

    println!("Enter first operand:");

    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read from STDIN.");
    // remove surrounding spaces.
    let num1 = num1.trim();
    // convert the string which is read into number.
    let a : f32 = num1.parse().unwrap();

    println!("Enter second operand:");

    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read from STDIN.");
    // remove surrounding spaces.
    let num2 = num2.trim();
    // convert the string which is read into number.
    let b : f32 = num2.parse().unwrap();

    println!("Enter an operator '+' or '-' or '*' or '/' or '%':");

    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("failed to read from STDIN.");
    let operator = operator.trim();

    // match statement in Rust is similar to,
    // but more powerful than the switch statement in C like languages.
    match operator {
        "+" => println!("Sum of two numbers {} {} is: {}",a,b,a+b),
        "-" => println!("Subtraction of two numbers {} {} is: {}",a,b,a-b),
        "*" => println!("Product of two numbers {} {} is: {}",a,b,a*b),
        "/" => println!("Quotient of two numbers {} {} is: {}",a,b,a/b),
        "%" => println!("Remainder of two numbers {} {} is: {}",a,b,a%b),
        _=> println!("Please enter a valid operator.")
    }
}
