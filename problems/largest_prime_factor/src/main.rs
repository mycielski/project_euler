fn main() {
    let mut n: usize = 600851475143;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }
    println!("{}", n);
}
