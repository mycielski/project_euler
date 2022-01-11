use rug::Integer;

fn main() {
    assert_eq!(lattice_paths(2), 6);
    println!("{}", lattice_paths(20));
}

fn lattice_paths(n: u32) -> Integer {
    Integer::from(2 * n).binomial(n)
}