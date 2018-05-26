//! #Problem 3
//! The prime factors of 13195 are 5, 7, 13 and 29.
//! What is the largest prime factor of the number 600851475143 ?

// Get largest prime factor of num 
pub fn  largest_prime_factor(num:u64) -> u64 {
    let mut value = num;
    let mut last_prime = 1;
    //find all prime factors
    for p in (2..num/2).filter(|x| is_prime(*x)) {
        if value <= 1 {
            break;
        }
        if value % p == 0 {
            value = value / p;
            last_prime = p;
        }
    }
    last_prime
}

fn is_prime(n:u64) -> bool{
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
    fn output_correct_value() {
        assert_eq!(6857, largest_prime_factor(600851475143));
    }
}