
mod fib;
mod fizzbuzz;
mod factorization;
mod primes;
mod palindromes;

use fib::*;
use fizzbuzz::*;
use factorization::*;
use palindromes::*;

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

#[cfg(test)]
mod tests {

}

fn main() {
    _01_product_of_threes_and_fives();
    _02_sum_of_fibs();
    _03_larges_prime_factor();
    _04_largest_palin_3digit_factors();
}
