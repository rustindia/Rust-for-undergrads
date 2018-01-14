use std::io;

/* Converts user input temperature 
 * from fahrenheit to celsius.
 */

fn read_float()->f32 {
	let mut num=String::new();
	
	io::stdin()
		.read_line(&mut num)
		.expect("error");
	
	num.trim().parse::<f32>()
		.unwrap()
}

fn main() {
    println!("Enter temperature in fahrenheit: ");
    let mut fahrenheit = read_float();
    let mut celsius = ((fahrenheit-32.0)*5.0)/9.0;
    println!("celsius is {}",celsius);
}
