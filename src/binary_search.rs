
/* Vinary search is a search algorithm that 
 * finds the position of a target value within
 * a sorted array.
 */ 

use std::io;

fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number.");

    num.trim()
        .parse::<i32>()
        .unwrap()
}

fn main() {
	println!("Enter number of elements:");
	let  n = read_int();
	println!("Enter {} numbers in increasing order: ",n);
	let mut v = vec!();
	for i in 0..n {
		println!("Enter {}th number: ",i+1);
		let x = read_int();
		v.push(x);
	}
	println!("Enter value to search: ");
	let search = read_int();
	let mut first:usize = 0;
	let mut last:usize = (n as usize)-1;
	let mut middle: usize = (first+last)/2;


	while first<=last {
		if v[middle]<search {
            // if element at the middle is less than
            // the element to be searched, search the
            // right half of the array.
			first = middle+1;
		} else if v[middle] == search {
			println!("{} found at location {}",search,middle+1);
			break;
		} else {
            // if the eleemnt to be searched is less than
            // the middle element, search the right half.
			last = middle-1;
		}
        // recalculate middle, because first or last has changed.
		middle =  (first + last)/2;
	}
	if first > last {
		println!("Could not find {} in the entered list.", search);
	}
}
