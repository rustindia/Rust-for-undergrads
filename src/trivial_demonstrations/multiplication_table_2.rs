/* Prints out the multiplication table for a given number.
 * Exemplifies the use of a for loop and a user defined function.
 */


fn main() {
     use std::io::{stdin};//HEADER FILE TO TAKE INPUT
     let mut num = String::new();
    println!("Enter the number to print multiplication table for:");
     stdin().read_line(&mut num).expect("Did not enter a correct string");//TAKING INPUTS AS ASTRING
     let n = num.trim().parse::<i32>().unwrap();//CONVERTING STRING TO INTEGER
   
    for i in 1..11 {
        println!("{}*{}={}",n,i,n*i);
    }
    
}
