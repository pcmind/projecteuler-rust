//! Problem 6
//! The sum of the squares of the first ten natural numbers is,
//! 
//! 1^2 + 2^2 + ... + 10^2 = 385
//! The square of the sum of the first ten natural numbers is,
//! 
//! (1 + 2 + ... + 10)^2 = 552 = 3025
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//! 
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.


pub fn sum_square_difference(max:u32) -> u32{

    //square of the sum of the first n natural numbers
    let v = ((max*(max+1))/2).pow(2);

    //The square of the sum of the first n natural numbers
    let mut v2 = 0;
    for i in 1..=max {
        v2 += i.pow(2);
    }
    v-v2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_solution() {
        assert_eq!(2640, sum_square_difference(10));
    }

    #[test]
    fn test_correct_result() {
        assert_eq!(25164150, sum_square_difference(100));
    }


}