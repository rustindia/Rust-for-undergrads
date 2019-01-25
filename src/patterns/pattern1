use std::io; 
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
    println!("Enter n");
    let n = read();
    // let size = read() as usize;
    for i in 0..n {
        for j in 0..i {
            print!("*");
        }
        println!("\n");
    }
}
     