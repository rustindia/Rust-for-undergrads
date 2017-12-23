use std::io;

fn main()
{
    println!("enter first operands:");

    let mut num1=String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("failed to read from stdin");
    let num1=num1.trim();
    let a : f32 = num1.parse().unwrap();

    println!("enter second operand:");

    let mut num2=String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("failed to read from stdin");
    let num2=num2.trim();
    let b : f32 = num2.parse().unwrap();

    println!("enter an operator '+' or '-' or '*' or '/' or '%':");

    let mut op=String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("failed to read from stdin");

    let op=op.trim();

    match op {
        "+" => println!("sum of two numbers {} {} is: {}",a,b,a+b),
        "-" => println!("subtraction of two numbers {} {} is:{}",a,b,a-b),
        "*" => println!("product of two numbers {} {} is: {}",a,b,a*b),
        "/" => println!("quotient of two numbers {} {} is:{}",a,b,a/b),
        "%" =>println!("remainder of two numbers {} {} is:{}",a,b,a%b),
        _=> println!("please enter correct operator")
    }
}
