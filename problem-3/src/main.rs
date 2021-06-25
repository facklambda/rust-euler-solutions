fn main () {
    let mut n = 600851475143_u64;
    let maxval = 600851475143_f64.sqrt();

    while n % 2 == 0 {
        println!("2");
        n /= 2;
    }

    for i in 3..maxval as u64 {
        
        while n % i == 0 {
            println!("{}", i);
            n /= i;
        }
    }
    if n > 2 {
        println!("{}", n);
    }
}