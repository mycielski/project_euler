fn main() {
    println!("{}", get_nth_prime(10001));
}

fn get_nth_prime(n: u64) -> u64 {
    let mut primes = vec![2];
    let mut i = 3;
    while primes.len() < n as usize {
        let mut is_prime = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
        i += 2;
    }
    primes[n as usize - 1]
}
