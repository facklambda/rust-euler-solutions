fn main() {
    //fib seq vars
    let mut prev = 1;
    let mut curr = 2;
    let mut next = 0;
    let mut sum = 2;

    while next < 4000000 {
        if next % 2 == 0 {
            sum += next;
            println!("Sum is now {0}, next in sequence is now {1}", sum, next);
        }
        else {
            println!("no change to sum");
        }
        next = curr + prev;
        prev = curr;
        curr = next;


    }
}
