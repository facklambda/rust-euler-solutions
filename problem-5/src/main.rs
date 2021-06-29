fn primedrive (n: u32, maxval: u32) -> u32 {
    let mut n = 20_u64;
    let maxval = 20_f64.sqrt();

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

fn main() {
    primedrive(n: u32, maxval: u32);
}