use std::io;
fn main()
{
let mut count = 1;
let mut sum = 1 ;
println!("Enter the value of x : ") ;

let mut num=String::new();
io::stdin()
.read_line(&mut num)
.expect("failed to read from stdin");
let num=num.trim();
let x: i32 = num.parse().unwrap();

println!("Enter the value of n : ") ;

let mut num1=String::new();
io::stdin()
.read_line(&mut num1)
.expect("failed to read from stdin");
let num1=num1.trim();
let n: i32 = num1.parse().unwrap();

while count <= n
{
sum = sum * x ;
count=count+1 ;
}
println!("The power of {}^{} is : {}", x, n, sum) ;
}
