fn main() {
    let mut v = Vec::new();
    for i in 1..=100 {
        v.push(i);
    }
    let sum: i32 = v.iter().sum();
    let square_of_sum: i32 = sum * sum;
    let sum_of_squares: i32 = v.iter().map(|x| x * x).sum();
    println!("{}", square_of_sum - sum_of_squares);
}