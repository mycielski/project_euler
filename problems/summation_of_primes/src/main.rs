fn main() {
    let primes = get_all_primes_below(2000000);
    let sum = primes.iter().fold(0, |sum, &x| sum + x);
    println!("{}", sum);
}

fn get_all_primes_below(n: usize) -> Vec<usize> {
    let mut primes = vec![];
    let mut is_prime = vec![true; n];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n {
        if is_prime[i] {
            primes.push(i);
            let mut j = i * i;
            while j < n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}
