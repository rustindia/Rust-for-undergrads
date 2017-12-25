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
         let mut imin:usize;
          println!("Enter the array elements");
          for _k in 0..size{
                   let x = read();
                   a.push(x);
           }
         
         for i in 0..size-2{
               imin=i;
               for j in i+1..size-1{
                     
                     if(a[j]<a[imin]){
                          imin=j;
                      }
                      
               }
                        temp=a[i];
                        a[i]=a[imin];
                        a[imin]=temp;        
          }
          println!("The sorted elements by selection sort are:");
          for l in 0..size{
                println!("{}",a[l]);
                
          }

}
