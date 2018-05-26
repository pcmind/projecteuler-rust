//! # Problem Solution 1
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//! 
//! Find the sum of all the multiples of 3 or 5 below 1000.

// First attempt to find solution. 
// Cost: O(n)
pub fn iterative_sum_of_divisible_by_3_and_5(max:u32) -> u32 {
    (0..max).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

//implementation based on the given optimal solution. 
//Cost: O(1)
pub fn optimal1_sum_of_divisible_by_3_and_5(max:u32) -> u32 {
    let target = max-1;
    let sum_of_divisible_by = |x, max| { let p = max/x; x * (p*(p+1)) / 2 };
    sum_of_divisible_by(3, target) + sum_of_divisible_by(5, target) - sum_of_divisible_by(15, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_correctly_provided_solution() {
        assert_eq!(23, iterative_sum_of_divisible_by_3_and_5(10));
    }

    #[test]
    fn iterative() {
        assert_eq!(233168, iterative_sum_of_divisible_by_3_and_5(1000));
    }

    #[test]
    fn optimal1() {
        assert_eq!(233168, optimal1_sum_of_divisible_by_3_and_5(1000));
    }
}