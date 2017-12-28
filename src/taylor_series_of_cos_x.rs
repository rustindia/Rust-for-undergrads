//rust program to find the sum of the given series:-
//Sum=1-x2/2! +x4/4!-x6/6!+x8/8!-x10/10! (here x2 means x power2)

use std::io;

fn main()
{
    let mut number=String::new();
    println!("Enter the value of x:");
    io::stdin().read_line(&mut number).expect("Error reading number");
    let x=number.trim().parse::<i64>().unwrap();

    let mut numerator=1;
    let mut denominator=1;
    let mut minus=1;
    let mut sum= 1_f64;

    for i  in 1..11
    {
        numerator=numerator*x;
        denominator=denominator*i;
        if i%2==0
        {
            if minus==1
            {
                sum=sum-((numerator/denominator)as f64);
                minus=0;
            }
            else if minus==0
            {

                sum=sum+((numerator/denominator)as f64);
                minus=1;
            }
        }
    }
    println!("When the value of x is {} the sum of the series is: {}",x,sum);
}
