use std::io;

/* Prints a user entered number in reverse.
 */

fn read_int()->u32 {
    let mut num=String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("error");

    num.trim().parse::<u32>()
        .unwrap()
}

fn main() {
    println!("Enter a Number");
    let mut a=read_int();
    let mut rev=0;

    while a>0 {
        let rem=a%10;
        a=a/10;
        rev=rev*10+rem;
    }

    println!("reverse is {}",rev);
}
