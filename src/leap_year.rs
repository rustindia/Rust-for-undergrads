//This program will tell you whether the given year is leap year or not 
use std::io;
fn read_int()->u32{                   // function to read values
	let mut num=String::new();
	
	io::stdin()
		.read_line(&mut num)
		.expect("error");
	
	num.trim().parse::<u32>()
		.unwrap()
}

fn main(){
let mut year=read_int();                               // taking input year from user
if year%4==0 && year%100!=0 || year%400 == 0{          // condition for a leap year
println!("{} is leap year",year);
}

else{
println!("{} is not leap year",year);
}
}
