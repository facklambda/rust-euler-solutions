fn main() {
    let mut n = 0;
    let mut sumsquare = 0;
    let mut squaresum = 0;

    for i in 1..=100 {
        sum_squares(i);
    }

}

fn sum_squares(sum: u64) -> u64 {
    sum += sum * sum
}

fn square_sum(n: u64) {
    n + 1;
}