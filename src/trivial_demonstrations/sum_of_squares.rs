use std::io;
fn main() {
    let _n:i32;
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("Error reading number");
    let _n = num.trim().parse::<i32>().unwrap();
    let _f:i32=(_n*(_n+1)*(2*_n+1))/6;
    println!("sum of squares of n natural numbers ={}",_f);
}
