use std::io;

/* Demonstration of various cool features offered by
 * println! macro.
 */

fn main(){
    let (first,last)=("Asif","Khan");
    println!("{}{}",first,last);
    println!("B : {:b} H : {:x}",10,20); // Acessing elements is various forms
    println!("{ten:>ws$}",ten="Asif",ws=5); // whitespace function
}
