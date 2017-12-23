use std::io;
fn main()
{
println!("enter your string");
let mut str=String::new();
io::stdin().read_line(&mut str); 
println!("string length is:{}",str.len());
}
