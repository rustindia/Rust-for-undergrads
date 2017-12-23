/* This program is to find the factors of a given number */
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
   println!("Enter a positive integer :");
   let  number=read_int();
   println!("Factors of {} are :",number);
   
   for i in 1..number+1
     {
       if number%i == 0
         {
            println!("{}",i);
         }
     }
}
