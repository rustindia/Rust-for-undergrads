use std::io; 

/* Read a list of numbers and 
 * sort using bubble sort algorithm.
 */

fn read()->i32 {
    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Error");
     num.trim()
        .parse::<i32>()
        .unwrap()
}

fn main() {

    println!("Enter the number of elements you wish to sort:");
    let size = read() as usize;
    let mut a = vec!();

    println!("Enter the {} numbers:", size);
    for _k in 0..size { 
       let x = read();
       a.push(x);
    }

    for i in 0..size-1 {
       for j in 0..size-i-1 {
             if a[j]>a[j+1] {
                 a.swap(j,j+1);
             }
       }         
    }
    println!("Here are the sorted numbers:");
    for l in 0..size {
        println!("{}",a[l]);
    }

}
