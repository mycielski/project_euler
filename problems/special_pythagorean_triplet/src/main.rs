fn main() {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for i in 1..1000 {
        for j in 1..1000 {
            let k = 1000 - i - j;
            if i * i + j * j == k * k {
                a = i;
                b = j;
                c = k;
            }
        }
    }
    println!("{}", a * b * c);
}
