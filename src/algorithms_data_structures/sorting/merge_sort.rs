use std::io;

fn merge<T: Copy + PartialOrd>(x1: &[T], x2: &[T], y: &mut [T]) {
	assert_eq!(x1.len() + x2.len(), y.len());
	let mut i = 0;
	let mut j = 0;
	let mut k = 0;
	while i < x1.len() && j < x2.len() {
		if x1[i] < x2[j] {
			y[k] = x1[i];
			k += 1;
			i += 1;
		} else {
			y[k] = x2[j];
			k += 1;
			j += 1;
		}
	}
	if i < x1.len() {
		y[k..].copy_from_slice(&x1[i..]);
	}
	if j < x2.len() {
		y[k..].copy_from_slice(&x2[j..]);
	}
}

fn merge_sort_rec<T: Copy + Ord>(x: &mut [T]) {
	let n = x.len();
	let m = n / 2;

	if n <= 1 {
		return;
	}

	merge_sort_rec(&mut x[0..m]);
	merge_sort_rec(&mut x[m..n]);

	let mut y: Vec<T> = x.to_vec();

	merge(&x[0..m], &x[m..n], &mut y[..]);

	x.copy_from_slice(&y);
}

fn main(){

	    println!("Enter the number of elements you wish to sort:");
		let size = read() as usize;
	    let mut a = vec!();

	    println!("Enter the {} numbers one after other:", size);
	    for _ in 0..size {
	       let x = read();
	       a.push(x);
	    }
		merge_sort_rec(&mut a);

	    println!("Here are the sorted numbers:");
	    for l in 0..size {
	        println!("{}",a[l]);
	    }
}

fn read()->i32 {
    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error");
     num.trim()
        .parse::<i32>()
        .unwrap()
}
