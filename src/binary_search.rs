/* binary search is a search algorithm that finds the position of a target value within a sorted array. */

use std::io;
fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");

    num.trim()
        .parse::<i32>()
        .unwrap()
}
fn main()
{
	println!("enter number of elements");
	let  n =read_int();
	println!("enter {} integer",n);
	let mut v=vec!();
	for i in 0..n
	{
		println!("enter {} number",i+1);
		let  x=read_int();
		v.push(x);
	}
	println!("enter value to search");
	let  search=read_int();
	let mut first:usize = 0;
	let mut last:usize = (n as usize)-1;
	let mut middle: usize=(first+last)/2;


	while first<=last
	{
		if  v[middle]<search
		{
			first = middle+1;
		}
		else if v[middle] == search
		{
			println!("{} found at location {}",search,middle+1);
			break;
		}
		else
		{
			last = middle-1;
		}
		middle=  (first + last)/2;
	}
	if first > last
	{
		println!("not found!");
	}
}
