//! #Problem 4
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//! Find the largest palindrome made from the product of two 3-digit numbers.
struct HalfPalindrome {
    half:u32,
    is_odd:bool
}

pub fn get_numbers_to_generate_max_palindrome_with_n_digits(digits:u32)  -> Option<(u32,u32)> {
    let max_oper = power_of_ten(digits) - 1;
    let max = first_palindrome_from(max_oper.pow(2));
    let mut p = first_half_of_palindrome(max);
    while p.half > 0 {
        let n = half_to_full_palindrome(&p);
        for i in (max_oper/2)..=max_oper {
            if n % i == 0 && i < max_oper && number_of_digits(n / i) == digits {
                return Some((i, n / i));
            }
        }
        p.half-=1;
        //TODO set is_odd correctly
    }
    None
}

fn power_of_ten(digits:u32) -> u32 {
    (10 as u32).pow(digits)
}

fn number_of_digits(v:u32) -> u32 {
    let mut size = 0;
    let mut v = v;
    while v > 0 {
        v = v /10;
        size+=1;
    }
    size
}

fn first_half_of_palindrome(v:u32) -> HalfPalindrome {
    let size = number_of_digits(v);
    let is_odd =  size % 2 != 0;
    let half = if is_odd {
        size /2 +1
    }else{
        size /2 
    };
    HalfPalindrome{ half: v / power_of_ten(half), is_odd} 
}

fn half_to_full_palindrome(p:&HalfPalindrome) -> u32{
    let mut res = p.half;
    let mut v = p.half;
    if p.is_odd {
        v = v / 10;
    }
    
    while v > 0 {
        res = res * 10 + (v % 10);
        v/=10;
    }
    res
}

fn first_palindrome_from(max:u32) -> u32 {
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

fn is_palindrome(n:u32) -> bool {
    let mut v = vec![];
    let mut r = n;
    while r > 0 {
        v.push(r % 10);
        r = r / 10;
    }
    let mut a = 0;
    let mut b = v.len()-1;
    while a<=b {
        if v[a] != v[b] {
            return false;
        }
        a+=1;
        b-=1;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_palindrome_is_correct() {
        assert_eq!(997799, first_palindrome_from(999*999));
    }

    #[test]
    fn max_palindrome_made_out_of_n_digit_number() {
        assert_eq!((91, 99), get_numbers_to_generate_max_palindrome_with_n_digits(2).unwrap());
        assert_eq!((913, 993), get_numbers_to_generate_max_palindrome_with_n_digits(3).unwrap());
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