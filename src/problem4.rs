//! #Problem 4
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! Find the largest palindrome made from the product of two 3-digit numbers.


/// Compute highest palindrome computed from number with n digits
pub fn get_highest_palindrome_generated_by_two_number_of_n_digits(digits:u32) -> Option<u32> {
    match get_numbers_to_generate_max_palindrome_with_n_digits(digits) {
        Some((x,y)) => Some(x*y),
        None => None
    }
}

/// Optimal solution given by projec teuler 
pub fn get_highest_palindrome_optimally_generated_by_two_number_of_3_digits() -> Option<u32> {
    let mut largest_palindrome = 0;
    let mut a = 999;
    while a >= 100 {
        let mut b = 999;
        while b >= a {
            if a*b <= largest_palindrome {
                break; //Since a*b is always going to be too small
            }
            if is_palindrome(a*b) {
                largest_palindrome = a*b;
            }
            b = b-1;
        }
        a = a-1;
    }
    Some(largest_palindrome)
}


/// brute force approach of get_highest_palindrome_optimally_generated_by_two_number_of_3_digits
pub fn brute_force_approach_3_digits() -> Option<u32>{
    let mut largest_palindrome = 0;
    let mut a = 100;
    while a <= 999 {
        let mut b = 100;
        while b <= 999 {
            if is_palindrome(a*b) && a*b > largest_palindrome {
                largest_palindrome = a*b
            }
            b = b+1;
        }
        a = a+1;
    }
    Some(largest_palindrome)
}

#[derive(Debug)]
struct Palindrome {
    //full value of palindrome
    p:u32,
    //size of the palindrome to avoid calculating it every time
    size:u32,
    //half part to enable simple subtraction to go dow in palindromes
    half:u32,
    //minimal value of half do detect when the size of the palindrome has changed
    min_half:u32,
    //palindrome is odd or even to know if last digit of half should be repeated
    is_odd:bool,
}

impl Palindrome {
    fn new(p:u32) -> Palindrome {
        if !is_palindrome(p) {
            panic!("Invalid palindrome {}", p);
        }
        let size = number_of_digits(p);
        let is_odd =  size % 2 != 0;
        let half = if is_odd {
            size /2 +1
        }else{
            size /2 
        };
        let min_half = power_of_ten(half)/10;
        Palindrome{ p, size, half: p / power_of_ten(size-half), min_half, is_odd}
    }
    
    //compute previous palindrome 
    fn mutate_to_prev(&mut self) {
        self.half-=1;
        if self.half < self.min_half {
            self.is_odd = !self.is_odd;
            self.size-=1;
            if !self.is_odd {
                self.min_half = self.min_half / 10;
            }else{
                self.half = self.half * 10 + 9;
            }
        }
        //TODO this odd value is incorrect. was valid for previous value of half
        self.p = half_to_full_palindrome(self.half, self.is_odd);
    }
}

/// Get (x,y), where x and y have the size of n digits that can produce the highest palindrome 
pub fn get_numbers_to_generate_max_palindrome_with_n_digits(digits:u32)  -> Option<(u32,u32)> {
    let max_oper = power_of_ten(digits) - 1;
    let max = first_palindrome_below(max_oper.pow(2));
    let mut p = Palindrome::new(max);
    while p.half > 0 {
        let n = p.p;
        for i in (max_oper/2)..=max_oper {
            if n % i == 0 && i < max_oper && number_of_digits(n / i) == digits {
                return Some((i, n / i));
            }
        }
        p.mutate_to_prev();
    }
    None
}

fn power_of_ten(digits:u32) -> u32 {
    (10 as u32).pow(digits)
}

/// Compute the number of digits 
fn number_of_digits(v:u32) -> u32 {
    let mut size = 0;
    let mut v = v;
    while v > 0 {
        v = v /10;
        size+=1;
    }
    size
}

/// Make a palindrome out of half a palindrome. if is_odd is true, last digit is to be 
/// placed twice in the center
fn half_to_full_palindrome(half:u32, is_odd:bool) -> u32{
    let mut res = half;
    let mut v = half;
    if is_odd {
        v = v / 10;
    }
    
    while v > 0 {
        res = res * 10 + (v % 10);
        v/=10;
    }
    res
}

// Find first palindrome below max number
fn first_palindrome_below(max:u32) -> u32 {
    let mut i = max;
    if i > 10 {
        //reduce the number of iteration required to get first palindrome
        while i > 10 {
            i/=10;
        }
        i = (max / 10)*10 + i
    }
    while i > 0 && !is_palindrome(i) {
        i-=10;
    }
    if !is_palindrome(i) {
        panic!("invalid palindrome: {}", i);
    }
    i
}


// verify if given n is a palindrome
fn is_palindrome(n:u32) -> bool {
    let mut n2 = n;
    let mut reverse = 0;
    while n2 > 0 {
        reverse = reverse * 10 + n2 % 10;
        n2 = n2/10;
    }
    reverse == n
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_palindrome_is_correct() {
        assert_eq!(997799, first_palindrome_below(999*999));
    }

    #[test]
    fn give_correct_solutions() {
        assert_eq!(9009, get_highest_palindrome_generated_by_two_number_of_n_digits(2).unwrap());
        assert_eq!(906609, get_highest_palindrome_generated_by_two_number_of_n_digits(3).unwrap());
        assert_eq!(906609, get_highest_palindrome_optimally_generated_by_two_number_of_3_digits().unwrap());
    }

    #[test]
    fn max_palindrome_made_out_of_n_digit_number() {
        assert_eq!((91, 99), get_numbers_to_generate_max_palindrome_with_n_digits(2).unwrap());
        assert_eq!((913, 993), get_numbers_to_generate_max_palindrome_with_n_digits(3).unwrap());
    }

    #[test]
    fn prev_return_only_palindrome_numbers() {
        let mut p =  Palindrome::new(999);
        for i in (0..9).rev() {
            p.mutate_to_prev();
            assert_eq!((90 + i) * 10 + 9, p.p);
        }
        p.mutate_to_prev();
        assert_eq!(898, p.p);
    }

    #[test]
    fn corner_cases() {
        let mut p =  Palindrome::new(1001);
        p.mutate_to_prev();
        assert_eq!(999, p.p);
        let mut p =  Palindrome::new(101);
        p.mutate_to_prev();
        assert_eq!(99, p.p);
        let mut p =  Palindrome::new(11);
        p.mutate_to_prev();
        assert_eq!(9, p.p);
    }

    #[test]
    fn detect_a_valid_palindrome() {
        assert_eq!(true, is_palindrome(9009));
        assert_eq!(true, is_palindrome(909));
        assert_eq!(true, is_palindrome(99));
    }

    #[test]
    fn detect_an_invalid_palindrome() {
        assert_eq!(false, is_palindrome(998001));
        assert_eq!(false, is_palindrome(9019));
        assert_eq!(false, is_palindrome(119));
        assert_eq!(false, is_palindrome(913));
    }
}