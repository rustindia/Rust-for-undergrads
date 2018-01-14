use std::io;

/* Prints the factorial of a user input
 * number, calculated using for loop.
 */

fn main() {
    println!("Enter a Number: ");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");

    let n = num.trim()
                .parse::<u32>()
                .unwrap();

    let mut factorial = 1;
    
    for i in 1..n+1 {
        factorial *= i;
    }

    println!("Factorial is {}",factorial);
}
