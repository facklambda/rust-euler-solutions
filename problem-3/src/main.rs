use std::f64;
fn main () {
    let n = 600851475143;
    let maxval = 600851475143.sqrt();

    while n % 2 == 0 {
        println!("2");
        n /= 2;
    }

    for i in 3..maxval {
        
        while n % i == 0 {
            println!("{0}", i);
            n /= i;
        }

    }
}