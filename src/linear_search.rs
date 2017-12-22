/* linear search is a program that checks every element in the list sequentially until the desired element is found */


use std::io;
fn read_input()->i32{
	let mut num=String::new();
	
	io::stdin()
		.read_line(&mut num)
		.expect("error");
	
	num.trim()
		.parse::<i32>()
		.unwrap()
}

fn main()
{
	let mut flag=false;
	
	println!("Enter the range of the array :");
	let size=read_input();
	
	
	println!("Enter {} elements (one in each line) :",size);
	let mut v=vec!();
	for _i in 0..size
	{
		let x = read_input();
		v.push(x);
	}
	
	println!("Enter the search element :");
	let search=read_input();
	
	
	for i in v
	{
		if search == i{
			flag=true;
			break;
                        
		}
	}
	
	if flag {
		println!("The number {} Found in the given array!",search);
	}
	else
	{
		println!("The number {} not found in the given array!",search);
	}
}
