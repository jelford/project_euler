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

    let bags: Vec<PrimeBag> = (2..21).map(|i| PrimeBag::from(&[i])).collect();
    let merged = bags.iter().fold(PrimeBag::empty(), |pb, next| pb.lowest_common_multiple(next));
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

fn _07_10001st_prime() {
    // p(n is prime) is about 1/ln(n)
    // ln(10001) is a bit more than 9, so shoudnt need to go above 10*10001
    let p = stream_of_primes(15*10001).nth(10000).unwrap();

    println!("07: {}", p);
}

fn _08_largest_seqprod_in_series() {
    let series: Vec<u64> = vec![7,3,1,6,7,1,7,6,5,3,1,3,3,0,6,2,4,9,1,9,2,2,5,1,1,9,6,7,4,4,2,6,5,7,4,7,4,2,3,5,5,3,4,9,1,9,4,9,3,4,9,6,9,8,3,5,2,0,3,1,2,7,7,4,5,0,6,3,2,6,2,3,9,5,7,8,3,1,8,0,1,6,9,8,4,8,0,1,8,6,9,4,7,8,8,5,1,8,4,3,8,5,8,6,1,5,6,0,7,8,9,1,1,2,9,4,9,4,9,5,4,5,9,5,0,1,7,3,7,9,5,8,3,3,1,9,5,2,8,5,3,2,0,8,8,0,5,5,1,1,1,2,5,4,0,6,9,8,7,4,7,1,5,8,5,2,3,8,6,3,0,5,0,7,1,5,6,9,3,2,9,0,9,6,3,2,9,5,2,2,7,4,4,3,0,4,3,5,5,7,6,6,8,9,6,6,4,8,9,5,0,4,4,5,2,4,4,5,2,3,1,6,1,7,3,1,8,5,6,4,0,3,0,9,8,7,1,1,1,2,1,7,2,2,3,8,3,1,1,3,6,2,2,2,9,8,9,3,4,2,3,3,8,0,3,0,8,1,3,5,3,3,6,2,7,6,6,1,4,2,8,2,8,0,6,4,4,4,4,8,6,6,4,5,2,3,8,7,4,9,3,0,3,5,8,9,0,7,2,9,6,2,9,0,4,9,1,5,6,0,4,4,0,7,7,2,3,9,0,7,1,3,8,1,0,5,1,5,8,5,9,3,0,7,9,6,0,8,6,6,7,0,1,7,2,4,2,7,1,2,1,8,8,3,9,9,8,7,9,7,9,0,8,7,9,2,2,7,4,9,2,1,9,0,1,6,9,9,7,2,0,8,8,8,0,9,3,7,7,6,6,5,7,2,7,3,3,3,0,0,1,0,5,3,3,6,7,8,8,1,2,2,0,2,3,5,4,2,1,8,0,9,7,5,1,2,5,4,5,4,0,5,9,4,7,5,2,2,4,3,5,2,5,8,4,9,0,7,7,1,1,6,7,0,5,5,6,0,1,3,6,0,4,8,3,9,5,8,6,4,4,6,7,0,6,3,2,4,4,1,5,7,2,2,1,5,5,3,9,7,5,3,6,9,7,8,1,7,9,7,7,8,4,6,1,7,4,0,6,4,9,5,5,1,4,9,2,9,0,8,6,2,5,6,9,3,2,1,9,7,8,4,6,8,6,2,2,4,8,2,8,3,9,7,2,2,4,1,3,7,5,6,5,7,0,5,6,0,5,7,4,9,0,2,6,1,4,0,7,9,7,2,9,6,8,6,5,2,4,1,4,5,3,5,1,0,0,4,7,4,8,2,1,6,6,3,7,0,4,8,4,4,0,3,1,9,9,8,9,0,0,0,8,8,9,5,2,4,3,4,5,0,6,5,8,5,4,1,2,2,7,5,8,8,6,6,6,8,8,1,1,6,4,2,7,1,7,1,4,7,9,9,2,4,4,4,2,9,2,8,2,3,0,8,6,3,4,6,5,6,7,4,8,1,3,9,1,9,1,2,3,1,6,2,8,2,4,5,8,6,1,7,8,6,6,4,5,8,3,5,9,1,2,4,5,6,6,5,2,9,4,7,6,5,4,5,6,8,2,8,4,8,9,1,2,8,8,3,1,4,2,6,0,7,6,9,0,0,4,2,2,4,2,1,9,0,2,2,6,7,1,0,5,5,6,2,6,3,2,1,1,1,1,1,0,9,3,7,0,5,4,4,2,1,7,5,0,6,9,4,1,6,5,8,9,6,0,4,0,8,0,7,1,9,8,4,0,3,8,5,0,9,6,2,4,5,5,4,4,4,3,6,2,9,8,1,2,3,0,9,8,7,8,7,9,9,2,7,2,4,4,2,8,4,9,0,9,1,8,8,8,4,5,8,0,1,5,6,1,6,6,0,9,7,9,1,9,1,3,3,8,7,5,4,9,9,2,0,0,5,2,4,0,6,3,6,8,9,9,1,2,5,6,0,7,1,7,6,0,6,0,5,8,8,6,1,1,6,4,6,7,1,0,9,4,0,5,0,7,7,5,4,1,0,0,2,2,5,6,9,8,3,1,5,5,2,0,0,0,5,5,9,3,5,7,2,9,7,2,5,7,1,6,3,6,2,6,9,5,6,1,8,8,2,6,7,0,4,2,8,2,5,2,4,8,3,6,0,0,8,2,3,2,5,7,5,3,0,4,2,0,7,5,2,9,6,3,4,5,0];
    
    let mut current_best: Option<PrimeBag> = None;

    for i in 0..(series.len()-13) {
        let elems = &series[i..i+13];
        let this_num = PrimeBag::from(elems);

        let new_best = match &current_best {
            Some(bst) => {
                if &this_num > bst {
                    Some(this_num)
                } else {
                    None
                }
            },
            None => {
                Some(this_num)
            }
        };

        if new_best.is_some() {
            current_best = new_best;
        }
    }

    let product: u64 = current_best.unwrap().into();
    println!("08: {}", product);
    
}

fn main() {
    _01_product_of_threes_and_fives();
    _02_sum_of_fibs();
    _03_larges_prime_factor();
    _04_largest_palin_3digit_factors();
    _05_smallest_evenly_divisible();
    _06_sum_of_squares_v_square_of_sum();
    _07_10001st_prime();
    _08_largest_seqprod_in_series();
}
