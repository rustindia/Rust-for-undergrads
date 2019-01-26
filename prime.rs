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
	
	let mut c=0;
	for i in 1..n+1
	{
		c=0;
		for j in 1..i+1
		{
			if i%j==0
			{
				c=c+1;
			}
		}
		if(c==2)
		{
			println!("{}",i);
		}
	}
}
