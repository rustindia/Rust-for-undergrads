use std::io;

/* An n digit number is an Amstrong number
 * if the sum of nth powers of its digits
 * equals that number
 */

fn main(){

    // TODO: Generalize this to n digits.

    println!("Enter a three digit integer: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read from STDIN.");
    let user_input = user_input.trim();
    if user_input.len() != 3 {
        panic!("The entered number is not three digits.");
    }
    let number: i32 = user_input.parse().unwrap();

    let mut number_copy = number;
    let mut result = 0;
    let mut ones_digit;
    
    while number_copy != 0 {
        // get one's digit of number using modulo arithmetic.
        ones_digit = number_copy%10;
        // add cube of that to a result variable.
        result += ones_digit.pow(3);
        // shift the number by one digit to the left,
        // so that we can continue extracting one's digit.
        // (discards any fractional part because this is integer division)
        number_copy /= 10;
        // when the last digit is shifted to left, number becomes 0
        // and the loop exits
    }

    if result == number {
        println!("{} is a three digit Armstrong number.",number);
    } else {
        println!("{} is not a three digit Armstrong number.",number);
    }

}
