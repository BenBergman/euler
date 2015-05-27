#[cfg(not(test))]
fn main() {
}


fn find_difference_of_square_of_sums_and_sums_of_squares(n: u64) -> u64 {
    let (x, y) = (1..n+1).map(|x| (x, x*x)).inspect(|y| println!("{:?}", y)).fold((0, 0), |(x, y), (a, b)| (x+a, y+b));
    (x*x) - y
}


#[test]
fn matches_example() {
    assert_eq!(find_difference_of_square_of_sums_and_sums_of_squares(10), 2640);
}


#[test]
fn matches_solution() {
    assert_eq!(find_difference_of_square_of_sums_and_sums_of_squares(100), 25164150);
}
