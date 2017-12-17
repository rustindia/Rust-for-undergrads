use std::io;

fn read_int()->u32{
	let mut num=String::new();
	
	io::stdin()
		.read_line(&mut num)
		.expect("error");
	
	num.trim().parse::<u32>()
		.unwrap()
}
fn main()
{
println!("enter two number");
let mut a=read_int();
let mut b=read_int();
let mut c;
c=a;
a=b;
b=c;
println!("a={},b={}",a,b);
}