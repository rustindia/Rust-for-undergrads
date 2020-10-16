use std::io;
fn main() 
{

	let mut n=String::new();
let mut m=String::new();
let mut user1=String::new();
let mut user2=String::new();
println!("Welcome Player 1!! What is your name?");
io::stdin().read_line(&mut user1).expect("No input given");
println!("1:rock\n2:paper\n3:scissors");
println!("Hello {}! What do you choose?",user1);
io::stdin().read_line(&mut n).expect("No input given");
let user_choice1 = n.trim()
                .parse::<u32>()
                .unwrap();
println!("Welcome Player 2!! What is your name?");
io::stdin().read_line(&mut user2).expect("No input given");
println!("1:rock\n2:paper\n3:scissors");
println!("Hello {}! What do you choose?",user2);
io::stdin().read_line(&mut m).expect("No input given");
let user_choice2 = m.trim()
                .parse::<u32>()
                .unwrap();

if user_choice1 == 1 && user_choice2 == 1 
{
    println!( "It was a tie!" );
} 
else if user_choice1 == 1 && user_choice2 == 3 {
    println!("The {} won! Better luck next time!",user1);
}
 else if user_choice1 == 2 && user_choice2 == 2 {
    println!("It was a tie!");
}
 else if user_choice1 == 2 && user_choice2 == 1 {
    println!("The {} won! Better luck next time!",user1);
}
else if user_choice1 == 3 && user_choice2 == 3 {
    println!( "It was a tie!");
}
 else if user_choice1 == 3 && user_choice2 == 2 {
    println!("The {} won! Better luck next time",user1);
} 
else
{
    println!("Congrats! You won {}!",user2);
}
}



