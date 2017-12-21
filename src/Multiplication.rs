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
println!("enter n value for table");
let mut n=read_int();
let mut i=1;
while i<=10 
{
println!("{}*{}={}",n,i,n*i);
i+=1;
}
}
