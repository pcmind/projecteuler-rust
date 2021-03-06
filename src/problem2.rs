//! #Problem 2
//! Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
//!
//! 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//!
//! By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.


/// Get largest prime factor of the number `count`
pub fn 	sum_of_even_fibonacci_numbers(count: u32) -> u32 {
    let mut sum = 2;
    let mut a = 1;
    let mut b = 2;
    loop {
        let r = a + b;
        a = b;
        b = r;
        if r % 2 == 0 {
            if r>count {
                break;
            }
            sum +=r;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_even_fibonacci_numbers_return_correct_sum() {
        assert_eq!(4613732, sum_of_even_fibonacci_numbers(4000000))
    }
}