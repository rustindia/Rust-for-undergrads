//PROGRAM TO COMPARE TO STRINGS
fn main() {
    use std::io::{stdin};//HEADER FILE TO TAKE INPUT
    let mut s=String::new();//ALLOCATING MEMORY FOR S
    let mut s1=String::new();//ALLOCATING MEMORY FOR S1
    print!("Please enter some text: ");
    stdin().read_line(&mut s).expect("Did not enter a correct string");
     print!("Please enter some text to compare with first: ");
    stdin().read_line(&mut s1).expect("Did not enter a correct string");
    
      if s1 == s //COMPARING 2STRINGS
     {
        println!("strings are same");
     }
     else
     {
        println!("strings are not same");
     }
    println!("i.e {} , {}",s1,s);
}
