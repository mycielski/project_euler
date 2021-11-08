fn main() {
    for i in 1..!0u128 {
        let triangle = get_nth_triangle_number(i);
        let divisors = divisors::get_divisors(triangle);
        if divisors.len() > 500 {
            println!("{}", triangle);
            break;
        }
    }
}

fn get_nth_triangle_number(n: u128) -> u128 {
    let mut i = 1;
    let mut sum = 0;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}
