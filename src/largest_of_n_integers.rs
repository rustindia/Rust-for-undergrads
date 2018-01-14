use std::io;

/* Reads a list of numbers and prints
 * the largest of them.
 */

fn read()->i32 {
    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("error");
    num.trim().parse::<i32>()
        .unwrap()
}
fn main() {
    let mut max:i32=0;
    println!("Enter the number of elements:")
    let size=read();
    println!("Enter {} numbers:", size)
    let mut v=vec!();
    for _i in 0..size
    {
        let x = read();
        if(x>max){
            max=x;
        }
        v.push(x);
    }
    println!("The greatest of entered numbers is: {}",max);
}
