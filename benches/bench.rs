// https://docs.rs/bencher/0.1.5/bencher/

#[macro_use]
extern crate bencher;
extern crate projecteuler;
use bencher::Bencher;
use projecteuler::problem1;
use projecteuler::problem2;
use projecteuler::problem3;

fn problem1_iterative(b: &mut Bencher) {
    b.iter(||{problem1::iterative_sum_of_divisible_by_3_and_5(500)});
}

fn problem1_optimal(b: &mut Bencher) {
    b.iter(||{problem1::optimal1_sum_of_divisible_by_3_and_5(500)});
}


fn problem2_sol1(b: &mut Bencher) {
    b.iter(||{problem2::sum_of_even_fibonacci_numbers(1000000)});
}

fn problem3_sol1(b: &mut Bencher) {
    b.iter(||{problem3::largest_prime_factor(600851475143)});
}

benchmark_group!(
    bench, 
    problem1_iterative, 
    problem1_optimal,
    problem2_sol1,
    problem3_sol1,
    );

benchmark_main!(bench);
