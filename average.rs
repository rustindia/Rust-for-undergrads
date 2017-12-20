/* Program to Calculate Sum and Average Using vector */

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

fn main(){
let mut sum=0;
println!("Enter the size of the array: ");
let size = read_int();
println!("Enter {} numbers of array: ",size);
let mut v=vec!();
 
 for _i in 0..size
{
 println!("Enter number {}: ",_i+1);
 let x = read_int();
 v.push(x);
 sum=sum+x;
}
let average = sum/size;
println!("The sum of {:?}  numbers is {}",v,sum);    
println!("The average of {:?}  numbers is {}",v,average);

}
