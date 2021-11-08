fn main() {
    let mut max_number: u128 = 0;
    let mut max_count: u128 = 0;
    for i in 1..1_000_000 {
        let mut n = i;
        let mut count = 1;
        while n != 1 {
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = 3 * n + 1;
            }
            count += 1;
        }
        if count > max_count {
            max_count = count;
            max_number = i;
        }
    }
    println!("{}", max_number);
}
