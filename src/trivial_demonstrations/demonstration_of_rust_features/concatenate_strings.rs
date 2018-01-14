use std::io;

/* Reads two strings.
 * Concatenates it and prints it.
 */

fn main()
{
    let mut str1 = String::new();
    io::stdin().read_line(&mut str1);
    str1 = String::from(str1.trim());
    let mut str2 = String::new();
    io::stdin().read_line(&mut str2);
    str2 = String::from(str2.trim());
    let mut str3 = str1+&str2;
    println!("final string is {}",str3);
}
