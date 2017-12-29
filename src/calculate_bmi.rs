use std::io;

/* Calculates Body Mass Index using the formula:
 *   bmi = weight in kg ÷(height in metre)^2
 */

 fn read() -> f32 {
    let mut num=String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("error");
    num.trim()
        .parse::<f32>()
        .unwrap()
 }

 fn main()
 {
    println!("Enter Weight in Kilograms: ");
    let weight_kg = read();
    
    println!("Enter Height in Centimetres: ");
    let height_cm = read();
    let height_m = height_cm/100.0;
     
    let bmi = weight_kg/height_m.powi(2);

    println!("Your BMI is {}",bmi);
 }
