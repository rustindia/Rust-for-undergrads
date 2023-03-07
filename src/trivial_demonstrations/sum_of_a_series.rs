use std::io;

/* Prints the sum of first 6 terms of the series:
 *      a_i = (-1)^i * x^(2i) * (2i)!
 *  where x is a number entered by user.
 */

fn main() {
    let mut num = String::new();

    println!("Enter Value of X:");
    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number.");

    let x = num.trim().parse::<i64>().unwrap();

    let mut sum: i64 = 0;
    for i in 0..6 {
        sum += i64::pow(-1, i) * x.pow(2 * i) * fact(2 * i as i64);
    }

    println!("Sum of series : {}", sum);
}

fn fact(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * fact(n - 1)
    }
}
