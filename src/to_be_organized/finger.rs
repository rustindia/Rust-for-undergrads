use std::io;

/* TODO: Figure out what this program does.
 */

fn read() -> i32 {
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("error");
    num.trim().parse::<i32>().unwrap()
}

fn main() {
    println!("Enter number");
    let a = read();

    let b = a % 8;
    if b == 1 {
        println!("{}", b);
    }
    if b == 5 {
        println!("{}", b);
    }
    if b == 0 || b == 2 {
        println!("2");
    }
    if b == 3 || b == 7 {
        println!("3");
    }
    if b == 4 || b == 6 {
        println!("4");
    }
}
