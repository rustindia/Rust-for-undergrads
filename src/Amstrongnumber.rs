use std::io;
fn main(){
    let mut originalnumber;
    let mut remainder;
    let mut result = 0;

println!("Enter a three digit integer ");
let mut num=String::new();
io::stdin()
.read_line(&mut num)
.expect("failed to read from stdin");
let num=num.trim();
let number: i32 = num.parse().unwrap();

originalnumber = number;

while originalnumber != 0
{
    remainder = originalnumber%10;
    result += remainder*remainder*remainder;
    originalnumber /= 10;
}

if result == number
    {println!("{} is an Armstrong number.",number);}
else
    {println!("{} is not an Armstrong number.",number);}

}
