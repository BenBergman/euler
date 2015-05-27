use std::u64::MAX;


#[cfg(not(test))]
fn main() {
}


struct Primes {
    current: u64,
}


impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        //println!("finding next prime from {} to {}", self.current+1, MAX);
        for i in (self.current+1..MAX) {
            //println!("testing {}", i);
            if is_prime(i) {
                //println!("{} is prime!", i);
                self.current = i;
                return Some(i);
            }
        }

        //println!("no prime found :(");
        None
    }
}


fn primes() -> Primes {
    Primes { current: 1 }
}


fn is_prime(n: u64) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else {
        let limit = (n as f64).sqrt() as u64;
        for i in primes().take_while(|&x| x <= limit) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}


#[test]
fn first_ten_primes_are_correct() {
    let mut iterator = primes();

    assert_eq!(iterator.next(), Some(2));
    assert_eq!(iterator.next(), Some(3));
    assert_eq!(iterator.next(), Some(5));
    assert_eq!(iterator.next(), Some(7));
    assert_eq!(iterator.next(), Some(11));
    assert_eq!(iterator.next(), Some(13));
    assert_eq!(iterator.next(), Some(17));
    assert_eq!(iterator.next(), Some(19));
    assert_eq!(iterator.next(), Some(23));
    assert_eq!(iterator.next(), Some(29));
}


#[test]
fn matches_example() {
    assert_eq!(primes().nth(5).unwrap(), 13);
}


#[test]
fn matches_solution() {
    assert_eq!(primes().nth(10_000).unwrap(), 104743);
}
