fn main() {
    let mut n: u32 = 0;

    for i in 1..=20 {
        while divvy_func(n, i) == false {
            n += 20;
            println!("n is: {}", n)
        }
        println!("i is: {}", i)
        i += 1;
    }
}

fn divvy_func(x: u32, y: u32) -> bool {
    x % y != 0
}
// 232792560
// 90716300
// 90716300
// 234317780