use std::io;

fn read_int()->f32{
	let mut num=String::new();
	
	io::stdin()
		.read_line(&mut num)
		.expect("error");
	
	num.trim().parse::<f32>()
		.unwrap()
}
fn main()
{
println!("enter temperature in fahrenheit");
let mut fahrenheit = read_int();
let mut celsius = ((fahrenheit-32.0)*5.0)/9.0;
println!("celsius is {}",celsius);
}
