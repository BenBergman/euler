#[cfg(not(test))]
fn main() {
    println!("Sum of all multiples of 3 or 5 below 1000: {}", sum_multiples_of_three_and_five(1000));
}

fn sum_multiples_of_three_and_five(max: u32) -> u32 {
    let mut sum = 0u32;

    for i in 1..max {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}


#[test]
fn matches_example() {
    assert_eq!(sum_multiples_of_three_and_five(10), 23);
}


#[test]
fn matches_solution() {
    assert_eq!(sum_multiples_of_three_and_five(1000), 233168);
}
