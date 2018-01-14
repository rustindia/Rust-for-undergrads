use std::io;
use::std::io::stdin;

/* Demonstration of vectors in Rust.
 */

fn main(){
    let mut a=vec![];
    let mut x;
    let array_size=read_int();
    for i in 0..array_size{
        x=read_int();
        a.push(x);
    }
    println!("{}",a[2]); // Acessing a  second element of a array.
}

fn read_int()->u32{
    let mut num=String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("error");

    num.trim().parse::<u32>()
        .unwrap()
}
