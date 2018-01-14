use std::io;

/* Demonstrates textbook example of swapping
 * two variables using a temporary variable.
 */

fn read_int()->u32{
    let mut num=String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("error");

    num.trim().parse::<u32>()
        .unwrap()
}

fn main() {
    println!("Enter two numbers: ");
    let mut a=read_int();
    let mut b=read_int();
    let mut temp;
    temp=a;
    a=b;
    b=temp;
    println!("a={},b={}",a,b);
}
