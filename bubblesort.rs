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
         let size=read() as usize;
         let mut a=vec!();
         let mut temp:i32=0;
         
          for _k in 0..size{
                   let x = read();
                   a.push(x);
           }
         
         for i in 0..size-1{
               for j in 0..size-i-1{
                     
                     if(a[j]>a[j+1]){
                        temp=a[j];
                        a[j]=a[j+1];
                        a[j+1]=temp;
                                            
                        
                     }
                      
               }         
          }
          for l in 0..size{
                println!("{}",a[l]);
                
          }

}
