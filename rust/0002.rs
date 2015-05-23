#[cfg(not(test))]
fn main() {
    for i in even_fibonacci_numbers_under_four_million() {
        println!("{}", i);
    }
    println!("sum: {}", even_fibonacci_numbers_under_four_million().iter().fold(0, |sum, i| sum + i));
}


fn even_fibonacci_numbers_under_four_million() -> Vec<u32> {
    fibonacci().take(35).filter(|&x| x < 4_000_000).filter(|&x| x % 2 == 0).collect()
}


struct Fibonacci {
    current: u32,
    next: u32,
}


impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let temp = self.current + self.next;
        self.current = self.next;
        self.next = temp;

        Some(self.current)
    }
}


fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 1,
        next: 1,
    }
}


#[test]
fn matches_solution() {
    assert_eq!(even_fibonacci_numbers_under_four_million().iter().fold(0, |sum, i| sum + i), 4613732);
}
