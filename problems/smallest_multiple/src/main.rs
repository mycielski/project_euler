use std::process::exit;

fn main() {
    let upper_bound = !0u128;
    // No need to use Euclid's algorithm, since we can just quickly check all numbers up to !0u128.
    for i in 1..upper_bound {
        for j in 1..=20 {
            if (i % j) != 0 {
                break;
            } else if j == 20 {
                println!("{}", i);
                exit(0);
            }
        }
    }
}
