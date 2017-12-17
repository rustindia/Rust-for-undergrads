use std::io::stdin;

/**
    GEOMETRIC PROGRESSION
    Takes space separated integer inputs x and n
    Calculates GP of number x upto n terms. GP ratio is assumed to be x.
    
    @input: [i32] [i32]
    @output: [i32]
    
    Outputs the GP as result    
*/

fn main(){        
    
    let mut input = String::new();          
    stdin().read_line(&mut input);   // READ LINE AS INPUT. 
    let mut iter = input.split_whitespace();   // SPLIT THE INTEGERS BASED ON WHITESPACE
    let mut x:i32;  // INITIALISING x as 32 bit integer
    let mut n:i32;  // INITIALISING n as 32 bit integer
    
    loop{
        
        x = iter.next().unwrap().parse::<i32>().unwrap();  // READS FIRST ELEMENT OF INPUT AS INT    
    
        n = iter.next().unwrap().parse::<i32>().unwrap();  // READS SECOND ELEMENT OF INPUT AS INT 
        
        if(n>=0) {break;}
        
        else{
            
            println!("Invalid input. n cannot be negative.")
            
        }
        
    }
    
    let mut sum = 0;
    
    loop{
        
        sum += x.pow(n as u32); // SUM
        
        n-=1; // LOOP COUNTER
        
        if (n == -1)  {break;} // BREAK LOOP
        
    }
    
    println!("{:?}", sum);  // PRINTS OUTPUT
    
}
