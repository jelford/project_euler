
use primes::*;

pub fn highest_prime_factor(target: u64) -> u64 {
    prime_factors(target).last().unwrap().clone()
}

pub fn prime_factors(target: u64) -> Vec<u64> {

    if target == 1 {
        return vec![1];
    }

    let seive_size = (target as f64).sqrt().ceil() as u64;
    let mut seive = stream_of_primes(seive_size);
    let mut res = Vec::with_capacity(10);
    let mut factor = 2;
    let mut t = target;

    loop {
        if t == 1 {
            return res;
        }

        if t == factor {
            res.push(factor);
            return res;
        }

        if t % factor == 0 {
            t = t / factor;
            res.push(factor);
        } else {
            if let Some(f) = seive.next() {
                factor = f;
            } else {
                res.push(t);
                return res;
            }
        }
    }
}


pub fn has_n_digit_factors(num: u64, digits: u32) -> bool {
    let (lower_lim, upper_lim) = (10_u64.pow(digits-1), 10_u64.pow(digits));

    let sqrt = (num as f64).sqrt();

    if (sqrt < lower_lim as f64)  || ((upper_lim as f64) < sqrt) {
        println!("sqrt: {}, lower: {}, upper: {}", sqrt, lower_lim, upper_lim);
        return false;
    }

    let (mut up, mut down) = (sqrt.ceil() as u64, sqrt.floor() as u64);

    let mut prod = up * down;
    while prod != num {
        if prod > num {
            down -= 1;
        } else {
            up += 1;
        }
        if down < lower_lim || up > upper_lim {
            return false;
        }
        prod = up * down;
    }
    return true;
}


#[cfg(test)]
mod tests {

    use super::highest_prime_factor;
    use super::prime_factors;
    use super::has_n_digit_factors;

    #[test]
    fn has_3_digit_factors_correctly_identifies_some_positive_cases() {
        assert!(has_n_digit_factors(123*456, 3));
        assert!(has_n_digit_factors(425*183, 3));
        assert!(has_n_digit_factors(740*777, 3));
        assert!(has_n_digit_factors(124*891, 3));
    }

    #[test]
    fn has_3_digit_factors_correctly_identifies_some_negative_cases() {
        assert!(!has_n_digit_factors(1, 3));
        assert!(!has_n_digit_factors(100, 3));
        assert!(!has_n_digit_factors(9999, 3));
        assert!(!has_n_digit_factors(10001, 3));
    }

    #[test]
    fn no_factor_of_a_three_digit_number_can_be_missed() {
        for i in 100..1000 {
            for j in 100..1000 {
                assert!(has_n_digit_factors(i * j, 3));
            }
        }
    }

    #[test]
    fn hpf_of_1_is_1() {
        assert_eq!(highest_prime_factor(1), 1);
    }

    #[test]
    fn hpf_returns_primes() {
        assert_eq!(highest_prime_factor(5), 5);
        assert_eq!(highest_prime_factor(1), 1);
        assert_eq!(highest_prime_factor(2), 2);
        assert_eq!(highest_prime_factor(13), 13);
    }

    #[test]
    fn hpf_finds_factors_of_large_numbers() {
        assert_eq!(highest_prime_factor(1003), 59);
    }

    #[test]
    fn hpf_finds_two_as_a_factor() {
        assert_eq!(highest_prime_factor(4), 2);
        assert_eq!(highest_prime_factor(8), 2);
        assert_eq!(highest_prime_factor(128), 2);
    }

    #[test]
    fn hpf_finds_three_as_a_factor() {
        assert_eq!(highest_prime_factor(9), 3);
        assert_eq!(highest_prime_factor(12), 3);
        assert_eq!(highest_prime_factor(81), 3);
    }

    #[test]
    fn prime_factors_of_some_primes_are_just_themselves() {
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(3), vec![3]);
        assert_eq!(prime_factors(5), vec![5]);
        assert_eq!(prime_factors(17), vec![17]);
        assert_eq!(prime_factors(29), vec![29]);
        assert_eq!(prime_factors(31), vec![31]);
    }

    #[test]
    fn prime_factors_can_be_found_for_small_numbers_with_only_two_factors() {
        assert_eq!(prime_factors(10), vec![2, 5]);
        assert_eq!(prime_factors(15), vec![3, 5]);
        assert_eq!(prime_factors(77), vec![7, 11]);
        assert_eq!(prime_factors(221), vec![13, 17]);
    }

    #[test]
    fn prime_factors_can_be_repeated() {
        assert_eq!(prime_factors(4), vec![2, 2]);
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
    }

    #[test]
    fn prime_factor_of_large_prime_can_be_found_as_itself() {
        assert_eq!(prime_factors(0xFFFFFFFB), vec![0xFFFFFFFB]);
    }
}
