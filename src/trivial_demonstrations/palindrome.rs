use std::io;
fn main(){
//here we take input
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("");
    //here we use len() to find length of string
         let half_len = num.len()/2;
         // here we use take() for taking a substring and eq for comparing
         //here we check palindrome
    if num.chars().take(half_len).eq(num.chars().rev().take(half_len))
    {println!("it is palindrome");}
    else
   { println!("it is not palindrome");}
}
