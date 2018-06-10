//! # Problem 5
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//! 
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?


// last_number is expected to be greater that 2
pub fn smallest_multiple(last_number:u32) -> u32{
    let mut primes = vec![];
    for i in (1..=last_number).rev() {
        if is_prime(i) {
            primes.push(i);
        }
    }
    let mut start = (if is_prime(last_number) { last_number-1 }else{last_number }) * primes[0];
    let mut is_ok;  
    loop {
        is_ok = true;
        for i in primes.iter() {
            if start % i != 0 {
                is_ok = false;
                break;
            }
        }
        if is_ok {
            for i in 1..last_number {
                if start % i != 0 {
                    is_ok = false;
                    break;
                }
            }
        }
        if !is_ok {
            start+=last_number;
            continue;
        }
        break;
    }
    start 
}

fn is_prime(n:u32) -> bool{
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_solution() {
        assert_eq!(2520, smallest_multiple(10));
    }

    #[test]
    fn test_corner_case() {
        assert_eq!(6, smallest_multiple(3));
    }
    
    #[test]
    fn test_correct_result() {
        assert_eq!(232792560, smallest_multiple(20));
    }
}