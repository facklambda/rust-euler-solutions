fn main() {
    let mut v = vec![1, 2];
    let mut sum: u32 = 0;


    while v[1] < 4000000 {
        if v[1] % 2 == 0 {
            sum += v[1];
            println!("{0}", sum)
        }
        v.push(v[0]+v[1]);
        v.remove(0);
    }
}
