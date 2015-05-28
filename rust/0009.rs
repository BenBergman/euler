#[cfg(not(test))]
fn main() {
}


fn find_product_of_pythagorean_triplet_with_sum(n: u64) -> Option<u64> {
    for a in 1..n-2 {
        for b in a+1..n-a {
            let c = n - a - b;
            if a*a + b*b == c*c {
                return Some(a*b*c);
            }
        }
    }
    None
}


#[test]
fn matches_example() {
    assert_eq!(find_product_of_pythagorean_triplet_with_sum(12), Some(60));
}


#[test]
fn matches_solution() {
    assert_eq!(find_product_of_pythagorean_triplet_with_sum(1000), Some(31_875_000));
}
