fn primedrive (n,maxval: u32,u32) -> u32 {
    let mut n = 50_u64;
    let maxval = 50_f64.sqrt();

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

fn is_palnindrome(num: u32) -> bool {
    return num.to_string() == num.to_string().chars().rev().collect::<String>();
    //shouts out to dnaka91 for the help :)
}

fn main() {
NEWS
}