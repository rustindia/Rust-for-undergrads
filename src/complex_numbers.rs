use std::io;
fn main() {
    println!("Enter values of First complex number");
    println!("Enter real value of First complex number");
    let mut real1 = String::new();
    io::stdin()
    .read_line(&mut real1)
    .expect("failed to read from stdin");
    let real1=real1.trim();
    let real1: i32 = real1.parse().unwrap();

    println!("Enter imaginary value of First complex number");
    let mut img1 = String::new();
    io::stdin()
    .read_line(&mut img1)
    .expect("failed to read from stdin");
    let img1=img1.trim();
    let img1: i32 = img1.parse().unwrap();

    println!("Enter values of Second complex number");
    println!("Enter real value of Second complex number");
    let mut real2 = String::new();
    io::stdin()
    .read_line(&mut real2)
    .expect("failed to read from stdin");
    let real2=real2.trim();
    let real2: i32 = real2.parse().unwrap();

    println!("Enter imaginary value of Second complex number");
    let mut img2 = String::new();
    io::stdin()
    .read_line(&mut img2)
    .expect("failed to read from stdin");
    let img2=img2.trim();
    let img2: i32 = img2.parse().unwrap();

    println!("Your First complex number is; {}+{}i",real1,img1 );
    println!("Your Second complex number is; {}+{}i",real2,img2 );

    let real3 = real1+real2;
    let img3= img1+img2;

    println!("Sum of First & Second complex number is; [{}]+[{}i]",real3,img3 );

    let real4 = real1*real2-img1*img2;
    let img4=real1*img2+real2*img1;

    println!("Product of First & Second complex number is; [{}]+[{}i]",real4,img4 );
}
