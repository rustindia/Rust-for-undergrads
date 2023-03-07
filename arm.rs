use std::io;
fn main() {
    println!("Enter a Number: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");
    let mut n = num.trim()
               .parse::<u32>()
             .unwrap();
	let t=n;
	let mut r;
	let mut s=0;
	while n>0
	{
		r=n%10;
		s=s+r*r*r;
		n=n/10;
	}
	if s==t {
		println!("{} is an armstrong number",s);
	}
	else {
		println!("{} is not an a armstrong number",s);
	}
}
