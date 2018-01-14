use std::io;

/* Prints a pyramid of triangular numbers.
 */


fn gap(n : i32) {
    for _i in 0..n+1 {
        print!("  ");
    }
}

fn main() {
    println!("Enter the n:");

    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("failed to read from stdin");
    let num=num.trim();
    let n: i32 = num.parse().unwrap();

    for i in 1..n+1 {
        gap(n-i);
        for _j in 0..i {
            print!("{}   ",i);
        }
        println!("");
    }
}
