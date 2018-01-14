use std::io;

fn read()->u32{
    let mut num=String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("error");
    num.trim().parse::<u32>()
        .unwrap()
}

fn main() {
    println!("User 1 enter the Number");
    let a=read();

    println!("User 2 enter the Number");
    let b=read();

    println!("User 3 enter the Number");
    let c=read();

    if a==b {
        println!("User 2 gussed it Right");
    } else {
        if a==c {
            println!("User 3 gussed it Right");
        } else {
            println!("Both gussed it Wrong");
        }
    }
}
