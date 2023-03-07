use std::io;

fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");

    num.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut n = 0;

    let mut flag = 0;

    println!("enter a number");

    n = read_int();

    for i in 2..n / 2 {
        if n % i == 0 {
            flag = 1;

            break;
        }
    }

    if flag == 0 {
        println!("is a prime");
    } else {
        println!("it is not a prime");
    }
}
