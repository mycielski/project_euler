fn main() {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < 4000000 {
        let c = a + b;
        a = b;
        b = c;
        if b % 2 == 0 {
            sum += b;
        }
    }
    println!("{}", sum + 2); // adding 2 because it was not checked for validity before
}