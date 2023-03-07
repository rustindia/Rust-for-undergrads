use std::io;

// This program is used to compute
// quotient and remainder for user input numbers

fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");

    num.trim().parse::<i32>().unwrap()
}

fn main() {
    println!("enter dividend:");
    let dividend = read_int(); // reading an integer for dividend
    println!("divisor:");
    let divisor = read_int(); //reading an integer for divisor
    let quotient = dividend / divisor; //calculating quotient
    let remainder = dividend % divisor; //calculating remainder
    println!("Quotient = {}", quotient);
    println!("Remainder = {}", remainder);
}
