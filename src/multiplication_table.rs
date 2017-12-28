use std::io;

/* Prints out the multiplication table for a given number.
 * Exemplifies the use of a for loop and a user defined function.
 */

fn read_int() -> u32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read from STDIN.");

    num.trim()
       .parse()
       .unwrap()
}
fn main() {
    println!("Enter the number to print multiplication table for:");
    let n = read_int();
    // 1..11 means 1 until 11 (excluding 11)
    for i in 1..11 {
        println!("{}*{}={}",n,i,n*i);
    }
}
