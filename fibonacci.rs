use std::io;

fn main() {
    println!("Enter value of n:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading input");

    let n = num.trim()
                .parse::<u32>()
                .unwrap();

    let (mut a, mut b) = (0,1);

    println!("First {} terms of Fibonacci Series", n);

    for _i in 0..n {
        println!("{}",a);
        let c = a+b;
        a = b;
        b = c;
    }
}