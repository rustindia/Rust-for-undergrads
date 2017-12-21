use std::io;

 fn read()->f32{
      let mut num=String::new();
      
        io::stdin()
                .read_line(&mut num)
                .expect("error");
             num.trim().parse::<f32>()
                       .unwrap()
 }
 fn main()
 {
     println!("Enter Weight in Kilograms");
    let a=read();
    
    println!("Enter Height in CentiMeters ");
    let b=read();
     
     let d=a/(b*b)*100.0*100.0;

     println!("Your BMI is {}",d);
 }
