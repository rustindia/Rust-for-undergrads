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
	println!("Enter a Number");
	let mut a=read_int();
	let mut sum=0;
	
	while a>0 {
		let rem=a%10;
		sum=sum+rem;
		a=a/10;
	}
	
	println!("sum is {}",sum);
}

