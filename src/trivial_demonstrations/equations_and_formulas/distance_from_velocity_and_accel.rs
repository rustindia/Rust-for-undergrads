use std::io;

/* Use equations of uniformly accelerated motion
 * to find the distance travelled given time, velocity and acceleration.
 */

fn main() {
    //variable a is acceleration in in(m/sec^2)
    //varible u is initial velocity in (m/sec)
    let mut s=0_f64;   //variable s is total distance travelled
    //variable t1 is the start of time interval in seconds
    //variable t2 is the end of time interval in seconds
    //variable i is used to iterate from t1 to t2


    println!("Enter the value of acceleration a :");

    let mut num1=String::new();
    io::stdin().read_line(&mut num1).expect("Error reading acceleration");
    let a=num1.trim().parse::<i64>().unwrap();

    println!("Enter the value of initial velocity u :");

    let mut num2=String::new();
    io::stdin().read_line(&mut num2).expect("Error reading initial velocity");
    let u=num2.trim().parse::<i64>().unwrap();


    println!("Enter the value of time interval t :");

    let mut time1=String::new();
    io::stdin().read_line(&mut time1).expect("Error reading t3");
    let t=time1.trim().parse::<i64>().unwrap();

    s=s+(((u*t)as f64)+(0.5*(a*t*t)as f64))as f64;


    println!("The total distance travelled by the vehicle is : {}",s );

}
