
mod fib;
mod fizzbuzz;
mod primes;

use fib::*;
use fizzbuzz::*;
use primes::*;

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


fn main() {
    _01_product_of_threes_and_fives();
    _02_sum_of_fibs();
    _03_larges_prime_factor();
}
