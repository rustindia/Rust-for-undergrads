use std::io;

/* Prints the length of a user entered string.
 */

fn main()
{
    println!("Enter your string: ");
    let mut str=String::new();
    io::stdin().read_line(&mut str);
    println!("String length is:{}",str.len());
}
