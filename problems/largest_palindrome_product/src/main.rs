fn main() {
    println!("{}", largest_palindrome_product());
}

fn largest_palindrome_product() -> u64 {
    let mut largest = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if product > largest && is_palindrome(product) {
                largest = product;
            }
        }
    }
    largest
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}