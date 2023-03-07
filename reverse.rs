use std::io;
fn read_int()->u32{
let mut num=string::new();
io::stdin()
	.read_line(&mut num)
	.expect("error");
num.trim().parse::<u32>()
	.unwrap()
}
fn main()
{
println!("enter a number");
let mut a=read_int();
let mut rev=0;
while a>0{
let rem=a%10;
a=a/10;
rev=rev*10+rem;
}
println!("reverse is {}",rev);
}