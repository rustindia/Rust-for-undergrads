use std::io;
fn main() {
    println!("Enter your first number");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("failed to read from stdin");
        let num1=num1.trim();
        let my_int1: i32 = num1.parse().unwrap();
        println!("Enter your second number");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("failed to read from stdin");
        let num2=num2.trim();
        let my_int2: i32 = num2.parse().unwrap();

    let mut remainder;
    let mut numerator;
    let mut denominator;
    let gcd;
    let  lcm;
    if my_int1 > my_int2
    {
        numerator = my_int1;
        denominator = my_int2;
    }
    else
    {
        numerator = my_int2;
        denominator = my_int1;
    }
    remainder = numerator % denominator;
    while remainder != 0
    {
        numerator   = denominator;
        denominator = remainder;
        remainder   = numerator % denominator;
    }
    gcd = denominator;
    lcm = my_int1 * my_int2 / gcd;
    println!("GCD of {} and {} is {}\n",my_int1,my_int2,gcd );
    println!("LCM of {} and {} is {}", my_int1, my_int2, lcm);
}
