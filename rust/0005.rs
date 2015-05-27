use std::u64::MAX;


#[cfg(not(test))]
fn main() {
}


fn find_smallest_divisible_number(a: u64, b: u64) -> u64 {
    (1..MAX)
        .filter(|x| {for y in (a..b) {
            if x % y != 0 {
                return false;
            }
        }
        true})
        .nth(0)
        .unwrap()
}


#[test]
fn matches_example() {
    assert_eq!(find_smallest_divisible_number(1, 10), 2520);
}


#[test]
fn matches_solution() {
    assert_eq!(find_smallest_divisible_number(1, 20), 232792560);
}
