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
          let mut b=vec!();
          for _i in 0..size{
                   let x = read();
                   a.push(x);
           }
          for _j in 0..size{
                   let y=read();
                   b.push(y);
                  
          }
          let mut c=vec!();
          for k in 0..size{
              c.push(a[k]+b[k]);
      	  }
	  println!("{:?}",c);      


}
