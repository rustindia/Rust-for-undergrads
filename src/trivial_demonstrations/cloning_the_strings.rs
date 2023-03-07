//CLONING THE INPUT STRINGS
 fn main() {
    use std::io::{stdin};
    let mut s=String::new();
    let mut s1=String::new();
    
    print!("Please enter some text: ");
    stdin().read_line(&mut s).expect("Did not enter a correct string");//TAKING INPUT TO STRING1
     s1 = s.clone();  //CLONING STRING1 TO STRING2
     println!("string 2 {}",s1);
}
