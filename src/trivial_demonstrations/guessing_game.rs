use std::io;

/* Let user input a guess.
 * Matches it to a predefined number.
 */
fn main() {

    // TODO: Randomize num
    let mut num=10;

    println!("Enter a number");
    while(num>0){
        let mut line=String::new();
        let input=stdin().read_line(&mut line);
        let guess: Option<i32>= input.ok().map_or(None,|_| line.trim().parse().ok());
        match guess{
            None=> println!("Please Enter a Number"),
            Some(x) if x == num => {
                println!("You Guessed it Right");
                break;
            }
            Some(x) if x < num => println!("TOO LOW"),
            Some(x)  if x > num => println!("TOO High"),
            Some(_) =>println!("Error")
        }
    }
}
