use std::time::{Instant};
//use std::thread::sleep;
fn main() {
    let mut i = 0;
    let mut f: f64 = 5200.0;
    let now = Instant::now();
    while i < 10000000 {
        f = 0.62*f + 1900.0;
        i+=1;
    }
    println!("{}", f);
    println!("{}", now.elapsed().as_millis());

}
