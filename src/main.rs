extern crate project_euler;

use project_euler::fib::*;
use project_euler::fizzbuzz::*;
use project_euler::factorization::*;
use project_euler::palindromes::*;
use project_euler::primes::*;
use project_euler::series::*;

fn _01_product_of_threes_and_fives() {
    let result: u64 = (0..1000).filter(is_product_of_3_or_5).sum();

    println!("01: {}", result);
}

fn _02_sum_of_fibs() {
    let res: u64 = fib_series().filter(|&x| x % 2 == 0).take_while(|&x| x < 4000000).sum();

    println!("02: {}", res);
}


fn _03_larges_prime_factor() {
    println!("03: {}", highest_prime_factor(600851475143))
}

fn _04_largest_palin_3digit_factors() {
    for p in palindromes_with_number_of_digits(6) {
        if has_n_digit_factors(p, 3) {
            println!("04: {}", p);
            return;
        }
    }
}

fn _05_smallest_evenly_divisible() {
    use std::iter::FromIterator;

    let bags: Vec<PrimeBag> = (2..21).map(|i| PrimeBag::from(prime_factors(i))).collect();
    let merged = bags.iter().fold(PrimeBag::empty(), |pb, next| pb.merged_with(next));
    let result: u64 = Vec::from_iter(merged).iter().product();

    println!("05: {}", result);
}

fn _06_sum_of_squares_v_square_of_sum() {

    let sum = sum_of_i(100_u64);
    let square_of_sum = sum.pow(2);

    let sum_of_squares = sum_of_squares_i(100);

    let diff = square_of_sum - sum_of_squares;
    println!("06: {}", diff);
}

fn main() {
    _01_product_of_threes_and_fives();
    _02_sum_of_fibs();
    _03_larges_prime_factor();
    _04_largest_palin_3digit_factors();
    _05_smallest_evenly_divisible();
    _06_sum_of_squares_v_square_of_sum();
}
