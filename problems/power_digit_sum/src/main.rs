#![feature(test)]
extern crate test;

fn main() {
    assert_eq!(power_digit_sum(15), 26);
    println!("{}", power_digit_sum(1000));
}

#[allow(clippy::needless_range_loop)]
fn large_power_digit_sum(exponent: u32) -> u32 {
    let mut v = Vec::with_capacity(!0u16 as usize);
    v.push(2);
    for _i in 2..=exponent {
        let mut carry = 0;
        for j in 0..v.len() {
            let product = v[j] * 2 + carry;
            v[j] = product % 10;
            carry = product / 10;
        }
        if carry > 0 {
            carry
                .to_string()
                .chars()
                .for_each(|c| v.push(c.to_digit(10).unwrap()));
        }
    }
    v.iter().sum()
}

#[allow(clippy::needless_return)]
fn power_digit_sum(exponent: u32) -> u32 {
    let base: u128 = 2;
    if let Some(power) = base.checked_pow(exponent) {
        return power
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum::<u32>();
    } else {
        return large_power_digit_sum(exponent); // here cargo clippy suggests incorrect syntax. must report issue.
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_large_power_digit_sum(b: &mut Bencher) {
        b.iter(|| large_power_digit_sum(1000));
    }
}
