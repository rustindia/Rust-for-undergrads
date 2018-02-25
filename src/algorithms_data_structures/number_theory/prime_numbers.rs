use std::io;

/* Prints all prime numbers between 1 and a user entered number.
*/

fn main() {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error Reading Number");

    let n = num.trim()
        .parse::<i32>()
        .unwrap();

    println!("Primes between 1 and {}",n);
    for i in 2..n+1 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn is_prime(n:i32) -> bool {
    for i in 2..(n.sqrt()) {
        if n%i==0 {
            return false
        }
    }
    true
}
