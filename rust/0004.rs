#[cfg(not(test))]
fn main() {
}


fn find_largest_palindrome_product(a: u32, b: u32) -> u32 {
    (1..a).map(|x| (1..b)
               .map(|y| x * y)
               .filter(|&z| is_palindrome(z))
               .max().unwrap_or(1))
        .max()
        .unwrap()
}


fn is_palindrome(n: u32) -> bool {
    let s = format!("{}", n);
    let mut r = String::new();
    for c in s.chars().rev() {
        r.push(c);
    }
    r == s
}


#[test]
fn is_palindrome_detects_valid_palindrome_numbers() {
    assert!(is_palindrome(3));
    assert!(is_palindrome(11));
    assert!(is_palindrome(878));
}


#[test]
fn is_palindrome_flags_bad_palindrome_numbers() {
    assert_eq!(is_palindrome(32), false);
    assert_eq!(is_palindrome(923), false);
    assert_eq!(is_palindrome(1000), false);
}


#[test]
fn matches_example() {
    assert_eq!(find_largest_palindrome_product(100, 100), 9009);
}


#[test]
fn matches_solution() {
    assert_eq!(find_largest_palindrome_product(1000, 1000), 906609);
}
