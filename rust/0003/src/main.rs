extern crate rand;
use rand::distributions::{Range, IndependentSample};

//const MAX: i64 = !0 as i64;
use std::u64::MAX;


#[cfg(not(test))]
fn main() {
    /*
    let random = Range::new(1, 6);
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        println!("rand: {}", random.ind_sample(&mut rng));
    }
    for i in (1..5).rev() {
        println!("{}", i);
    }
    let num = 600_851_475_143;
    println!("Largest prime factor of {}: {}", num, largest_prime_factor(num));
    */

    for i in 0..70 {
        println!("{} is prime: {}", i, is_prime(i));
    }
    println!("{} is prime: {}", 9, is_prime(9));
}


fn largest_prime_factor(num: u64) -> u64 {
    let mut max = 1;
    let mut limit = (num as f64).sqrt() as u64;
    let mut leftover = num;
    for p in primes() {
        println!("testing if {} is a factor of {}", p, leftover);
        if leftover % p == 0 {
            println!("  it is!");
            max = p;
            leftover /= p;
            limit = (leftover as f64).sqrt() as u64;
        }
        //println!("max prime check...");
        if p > limit {
            break;
        }
    }
    if leftover > max {
        max = leftover;
    }
    max
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


#[allow(dead_code, unused_variables)]
fn is_prime_full_aks(n: u64) -> bool {
    let nf = n as f64;
    for b in 2..(nf.log2() as u64) {
        let a = nf.powf((1/b) as f64);
        if a == a.trunc() {
            return false;
        }
    }

    let maxk = nf.log2().powf(2.0).floor();
    let mut next_r;
    let mut r = 2;
    for rl in 2..MAX {
        next_r = false;
        for k in 1..maxk.floor() as u64+1 {
            let rem = nf.powf(k as f64) % rl as f64;
            next_r = rem == 1.0 || rem == 0.0;
            if next_r {
                break;
            }
        }
        if !next_r {
            break;
        }
        r = rl;
    };

    for a in (2..r).rev() {
    }
    true
}


#[allow(dead_code)]
fn is_prime_simplified_aks(n: i64) -> bool {
    if n < 2 {
        false
    } else {
        let c = aks_coefficients(n);
        println!("aks_coefficients: {:?}", c);
        (1..(c.len() - 1) / 2 + 1).all(|i| (c[i] % n) == 0)
    }
}


#[allow(dead_code, unused_variables)]
fn is_prime_miller_rabin(n: u64) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else if n == 3 {
        true
    } else {
        let random = Range::new(2, n-1);
        let mut rng = rand::thread_rng();
        let mut d = n - 1;
        let mut s = 0;
        while d % 2 == 0 {
            d /= 2;
            s += 1;
        }
        println!("starting witness loop...");
        'witness_loop: for certainty in (0..256).filter(|a| a % 2 == 0) {
            print!("picking random number");
            let a = random.ind_sample(&mut rng);
            println!(" {}", a);
            let mut x = a.pow(d as u32) % n;
            println!("x: {}", x);
            if x == 1 || x == n - 1 {
                continue;
            }
            for _ in 1..s {
                x = x.pow(2) % n;
                println!("new x: {}", x);
                if x == 1 {
                    return false;
                } else if x == n - 1 {
                    continue 'witness_loop;
                }
            }
            return false;
        }
        true
    }
}


fn is_prime(n: u64) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else if n == 3 {
        true
    } else {
        let limit = (n as f64).sqrt() as u64;
        for i in primes().take_while(|&x| x <= limit) {
            if n % i == 0 {
                println!("  it is!");
                return false;
            }
        }
        true
    }
}


fn aks_coefficients(k: i64) -> Vec<i64> {
    if k == 0 {
        vec![1i64]
    } else {
        let zero = Some(0i64);
        (1..k).fold(vec![1i64, -1], |r, _| {
            let a = r.iter().chain(zero.iter());
            let b = zero.iter().chain(r.iter());
            a.zip(b).map(|(x, &y)| x-y).collect()
        })
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
    assert_eq!(largest_prime_factor(13195), 29);
}


#[test]
fn matches_solution() {
    assert_eq!(largest_prime_factor(600_851_475_143), 6857);
}
