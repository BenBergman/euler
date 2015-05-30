#[cfg(not(test))]
fn main() {
}


struct TriangleNums {
    index: u64,
    current: u64,
}


impl Iterator for TriangleNums {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current += self.index;
        self.index += 1;

        Some(self.current)
    }
}


fn triangle_numbers() -> TriangleNums {
    TriangleNums { index: 1, current: 0 }
}


#[test]
fn first_ten_triangle_numbres_are_correct() {
    let mut iterator = triangle_numbers();

    assert_eq!(iterator.next(), Some(1));
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(iterator.next(), Some(6));
    assert_eq!(iterator.next(), Some(10));
    assert_eq!(iterator.next(), Some(15));
    assert_eq!(iterator.next(), Some(21));
    assert_eq!(iterator.next(), Some(28));
    assert_eq!(iterator.next(), Some(36));
    assert_eq!(iterator.next(), Some(45));
    assert_eq!(iterator.next(), Some(55));
}


fn find_factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();

    for i in 1..n+1 {
        if n % i == 0 {
            factors.push(i);
        }
    }

    factors
}


#[test]
fn find_factors_spot_check() {
    assert_eq!(find_factors(1), vec![1]);
    assert_eq!(find_factors(2), vec![1, 2]);
    assert_eq!(find_factors(7), vec![1, 7]);
    assert_eq!(find_factors(28), vec![1, 2, 4, 7, 14, 28]);
}


#[test]
fn matches_example() {
    assert_eq!(triangle_numbers().find(|&x| find_factors(x).len() > 5).unwrap(), 28);
}


#[test]
fn matches_solution() {
    assert_eq!(triangle_numbers().find(|&x| find_factors(x).len() > 500).unwrap(), 76_576_500);
}
