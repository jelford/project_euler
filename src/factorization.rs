
use primes::*;

pub fn highest_prime_factor(target: u64) -> u64 {
    if target == 1 {
        return 1;
    }

    let seive_size = (target as f64).sqrt().ceil() as u64;
    let mut seive = stream_of_primes(seive_size);

    let mut t = target;
    let mut factor: u64 = 2;

    loop {
        if t == factor || t < factor {
            return t;
        }

        if t % factor == 0 {
            t = t / factor;
        } else {
            factor = seive.next().unwrap();
        }
    }
}


#[cfg(test)]
mod tests {

    use super::highest_prime_factor;


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

}
