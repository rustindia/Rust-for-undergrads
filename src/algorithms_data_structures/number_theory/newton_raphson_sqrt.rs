// Implementing Newton Raphson method to find square root
fn sqrt(x:f64, y:u8)-> f64 {
    let mut r = x.clone();

    match (x < 0.0, y > 11) {
        (false, false) => {

            // Set precision
            let precision:f64 = 10f64.powi(-1*(y as i32));

            while (x - r * r).abs() > precision {
                r = (r + x / r) / 2.0;
            }
        }
        (true, false) => panic!("Trying to find imaginary roots!"),
        (false, true) => panic!("Trying to find root of too much precision!"),
        (true, true) => panic!("Trying to find imaginary roots to a very large precision!")
    };

    r
}

fn main() {
    // Test with a random value
    let i = 3.14;
    let j = 3;
    println!("The square root of {} for {} decimal places is: {}", i, j, sqrt(i as f64, j));
}
