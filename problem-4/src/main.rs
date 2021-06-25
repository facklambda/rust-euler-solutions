fn main() {
    let mut max: u32 = 0;
    for x in 100..=999 {
        for y in 100..=999 {
            if is_palnindrome(x * y) && max < x * y {
                max = x * y;
                println!("{}", max)
            }
        }
    }
}

fn is_palnindrome(num: u32) -> bool {
    return num.to_string() == num.to_string().chars().rev().collect::<String>();
    //shouts out to dnaka91 for the help :)
}
