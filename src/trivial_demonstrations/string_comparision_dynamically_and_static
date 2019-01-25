//PROGRAM TO COMPARE TO STRINGS WITH ONE STATIC AND ONE DYNAMIC INPUT
fn main() {
    use std::io::{stdin};//HEADER FILE TO TAKE INPUT
    let s="HELLO\n";//ALLOCATING MEMORY FOR S
      let char_vec: Vec<char> = s.chars().collect();
    let mut s1=String::new();//ALLOCATING MEMORY FOR S1
    //print!("Please enter some text: ");
    //stdin().read_line(&mut s).expect("Did not enter a correct string");
    //s = "HELLO ";
     print!("Please enter some text to compare with first: ");
    stdin().read_line(&mut s1).expect("Did not enter a correct string");
      let char_vec1: Vec<char> = s1.chars().collect();
    
      if char_vec == char_vec1//COMPARING 2STRINGS
     {
        println!("strings are same");
     }
     else
     {
        println!("strings are not same");
     }
    println!("i.e {}  {}",s1,s);
}
