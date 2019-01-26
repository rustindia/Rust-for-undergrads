use std::io;
fn main() {
    let n:i32;
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("Error reading number");
    let n = num.trim().parse::<i32>().unwrap();
    let f:i32=(n*(n+1)*(2*n+1))/6;
    println!("sum of squares of n natural numbers ={}",f);
}
