use std::io; 
fn read()->i32{
      let mut num=String::new();
      
        io::stdin()
                .read_line(&mut num)
                .expect("error");
             num.trim().parse::<i32>()
                       .unwrap()
}
fn main(){
         println!("Enter the array size:");
         let size=read() as usize;
         let mut a=vec!();
         let mut temp:i32=0;
         let mut blank:usize=0;
          println!("Enter the array elements");
          for _k in 0..size{
                   let x = read();
                   a.push(x);
           }
         
         for i in 0..size{
               temp=a[i];
               blank=i;
            
               while blank>0 && temp<a[blank-1]{
                     a[blank]=a[blank-1];
                     blank=blank-1;
                   }
                    a[blank]=temp;         
          }
          println!("The sorted elements by insertion sort are:");
          for l in 0..size{
                println!("{}",a[l]);
                
          }

}
