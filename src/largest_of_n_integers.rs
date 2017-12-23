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
   let mut max:i32=0;
   let size=read();
   let mut v=vec!();
   for _i in 0..size
	{
		let x = read();
		if(x>max){
		max=x;
		}
		v.push(x);
         }       
println!("{}",max);


}
