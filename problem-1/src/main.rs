fn main() {
    let mut x = 0;
    let mut sum = 0;
    while x < 1000 {
        if x % 3 == 0 {
            sum += x;
            println!("Sum is now {0}, X is now {1}", sum, x);
        }
        else if x % 5 == 0 {
            sum += x;
            println!("Sum is now {0}, X is now {1}", sum, x);
        }
        else {
            println!("Sum is now {0}, X is now {1}", sum, x);
        }
        x += 1;
    }

}