extern crate primal;

fn main () {
    let sieve = primal::Sieve::new(600851475143);
    println!("{:?}", sieve.factor(600851475143));
}