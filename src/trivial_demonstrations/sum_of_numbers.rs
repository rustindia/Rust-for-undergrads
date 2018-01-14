use std::io;

/* Reads a number N from STDIN.
 * Prints the sum of first N natural numbers.
 */

fn read_int()->u32{
    let mut num=String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("error");

    num.trim().parse::<u32>()
        .unwrap()
}
fn main(){
    let x=read_int();
    println!("{}",x*(x+1)/2);
}
