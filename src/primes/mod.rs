use super::factorization::prime_factors;
use std::cmp::{Ordering, PartialEq, PartialOrd};

pub struct StreamOfPrimes {
    seed_primes: Vec<u64>,
    upper_bound: u64,
    current_index: usize,
    segment_size: usize,
    next_segment_floor: u64,
}

fn primes_from_next_seive_segment(floor: u64, size: usize, seed_primes: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::with_capacity(size / 2);

    let mut new_segment = vec![true; size];
    let upper_bound = floor + size as u64;
    for i in seed_primes.iter().take_while(|&x| x.pow(2) < upper_bound) {
        let p = i.clone() as f64;
        let start_at = (floor as f64 / p).ceil() as u64;

        for j in start_at.. {
            let non_prime = i * j;
            if non_prime >= floor + size as u64 {
                break;
            }

            let target = (non_prime - floor) as usize;
            new_segment[target] = false;
        }
    }

    for (pos, &is_prime) in new_segment.iter().enumerate() {
        if is_prime {
            result.push(pos as u64 + floor)
        }
    }

    result
}

impl Iterator for StreamOfPrimes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.seed_primes.len() {
            let current_index = self.current_index;
            self.current_index += 1;
            Some(self.seed_primes[current_index])
        } else if self.next_segment_floor >= self.upper_bound {
            None
        } else {
            let mut next_segment = primes_from_next_seive_segment(
                self.next_segment_floor,
                self.segment_size,
                &self.seed_primes,
            );

            self.seed_primes.append(&mut next_segment);

            self.next_segment_floor = self.next_segment_floor + self.segment_size as u64;

            self.next()
        }
    }
}

pub fn stream_of_primes(upper_bound: u64) -> StreamOfPrimes {
    let (segment_size, expected_number_of_primes) = if upper_bound > 1000 {
        let fupper_bound = upper_bound as f64;
        (
            fupper_bound.sqrt().ceil() as usize,
            (fupper_bound / fupper_bound.log(2_f64)).ceil() as usize,
        )
    } else {
        ((upper_bound + 1) as usize, (upper_bound / 2) as usize)
    };

    let mut initial_seive = vec![true; segment_size + 1];
    initial_seive[0] = false;
    initial_seive[1] = false;

    let mut seed_primes = Vec::with_capacity(expected_number_of_primes);

    for i in 2..segment_size {
        if initial_seive[i] {
            for j in i..(segment_size / i) + 1 {
                initial_seive[i * j] = false
            }
            seed_primes.push(i as u64);
        }
    }

    StreamOfPrimes {
        seed_primes: seed_primes,
        upper_bound: upper_bound,
        current_index: 0,
        segment_size: segment_size,
        next_segment_floor: segment_size as u64,
    }
}

#[derive(Debug)]
pub struct PrimeBag {
    primes: Vec<u64>,
}

impl PrimeBag {
    pub fn lowest_common_multiple(&self, other: &PrimeBag) -> PrimeBag {
        let mut pos_self = 0;
        let mut pos_other = 0;
        let mut new_primes = Vec::with_capacity(self.primes.len() + other.primes.len());
        loop {
            if pos_self >= self.primes.len() || pos_other >= other.primes.len() {
                break;
            }
            if self.primes[pos_self] == other.primes[pos_other] {
                new_primes.push(self.primes[pos_self]);
                pos_self += 1;
                pos_other += 1;
            } else if self.primes[pos_self] < other.primes[pos_other] {
                new_primes.push(self.primes[pos_self]);
                pos_self += 1;
            } else if self.primes[pos_self] > other.primes[pos_other] {
                new_primes.push(other.primes[pos_other]);
                pos_other += 1;
            }
        }

        new_primes.extend_from_slice(&self.primes[pos_self..]);
        new_primes.extend_from_slice(&other.primes[pos_other..]);

        PrimeBag { primes: new_primes }
    }

    pub fn from(factors: &[u64]) -> PrimeBag {
        let mut prime_bag = Self::empty();

        for factor in factors {
            if factor == &0u64 {
                return Self::empty();
            }
            let primes_of_this = prime_factors(*factor);
            let primes_of_this = PrimeBag {
                primes: primes_of_this,
            };
            prime_bag = prime_bag.multiply(&primes_of_this);
        }

        prime_bag
    }

    pub fn multiply(&self, other: &Self) -> PrimeBag {
        let mut pos_self = 0;
        let mut pos_other = 0;
        let mut new_primes = Vec::with_capacity(self.primes.len() + other.primes.len());
        loop {
            if pos_self >= self.primes.len() || pos_other >= other.primes.len() {
                break;
            }
            if self.primes[pos_self] == other.primes[pos_other] {
                new_primes.push(self.primes[pos_self]);
                new_primes.push(other.primes[pos_other]);
                pos_self += 1;
                pos_other += 1;
            } else if self.primes[pos_self] < other.primes[pos_other] {
                new_primes.push(self.primes[pos_self]);
                pos_self += 1;
            } else if self.primes[pos_self] > other.primes[pos_other] {
                new_primes.push(other.primes[pos_other]);
                pos_other += 1;
            }
        }

        new_primes.extend_from_slice(&self.primes[pos_self..]);
        new_primes.extend_from_slice(&other.primes[pos_other..]);

        PrimeBag { primes: new_primes }
    }

    pub fn divide(&self, other: &Self) -> PrimeBag {
        let mut pos_self = 0;
        let mut pos_other = 0;
        let mut new_primes = Vec::with_capacity(self.primes.len());
        loop {
            if pos_self >= self.primes.len() || pos_other >= other.primes.len() {
                break;
            }

            if self.primes[pos_self] == other.primes[pos_other] {
                pos_self += 1;
                pos_other += 1;
            } else if self.primes[pos_self] < other.primes[pos_other] {
                new_primes.push(self.primes[pos_self]);
                pos_self += 1;
            } else if self.primes[pos_self] > other.primes[pos_other] {
                pos_other += 1;
            }
        }

        new_primes.extend_from_slice(&self.primes[pos_self..]);

        PrimeBag { primes: new_primes }
    }

    pub fn empty() -> PrimeBag {
        PrimeBag { primes: vec![] }
    }

    pub fn iter(&self) -> PrimeBagIterator {
        PrimeBagIterator {
            current_index: 0,
            primes: &self.primes,
        }
    }
}

impl Into<u64> for PrimeBag {
    fn into(self) -> u64 {
        return self.primes.iter().fold(1, |acc, &x| acc * x);
    }
}

impl IntoIterator for PrimeBag {
    type Item = u64;
    type IntoIter = ::std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.primes.into_iter()
    }
}

pub struct PrimeBagIterator<'a> {
    current_index: usize,
    primes: &'a Vec<u64>,
}

impl<'a> Iterator for PrimeBagIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.primes.len() <= self.current_index {
            None
        } else {
            let ret = self.primes[self.current_index];
            self.current_index += 1;
            Some(ret)
        }
    }
}

impl PartialEq for PrimeBag {
    fn eq(&self, other: &Self) -> bool {
        let mut left = self.iter();
        let mut right = other.iter();

        let mut left_i = left.next();
        let mut right_i = right.next();

        while (left_i == right_i) && left_i.is_some() {
            left_i = left.next();
            right_i = right.next();
        }

        return left_i == right_i;
    }
}

impl PartialOrd for PrimeBag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let left = self.divide(other);
        let right = other.divide(self);

        if left.primes.len() == 0 && right.primes.len() > 0 {
            return Some(Ordering::Less);
        } else if right.primes.len() == 0 && left.primes.len() > 0 {
            return Some(Ordering::Greater);
        }

        let left_total: u64 = left.into();
        let right_total: u64 = right.into();

        return if left_total > right_total {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        };
    }
}

#[cfg(test)]
mod tests {
    use super::stream_of_primes;
    use super::PrimeBag;

    mod stream_of_primes {
        use super::*;

        #[test]
        fn stream_of_primes_yields_some_primes() {
            let mut s = stream_of_primes(100);
            assert_eq!(s.next(), Some(2));
            assert_eq!(s.next(), Some(3));
            assert_eq!(s.next(), Some(5));
            assert_eq!(s.next(), Some(7));
        }

        #[test]
        fn stream_of_primes_yields_primes_including_max() {
            let mut s = stream_of_primes(7);
            assert_eq!(s.next(), Some(2));
            assert_eq!(s.next(), Some(3));
            assert_eq!(s.next(), Some(5));
            assert_eq!(s.next(), Some(7));
        }

        #[test]
        fn stream_of_primes_finished_at_its_upper_bound() {
            let r = stream_of_primes(100).collect::<Vec<_>>();
            assert_eq!(
                r,
                vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97,
                ]
            );
        }

        #[test]
        fn streame_of_primes_remains_accurrate_for_large_numbers() {
            let r = stream_of_primes(15000000).nth(3939).unwrap();
            assert_eq!(r, 37189);
        }
    }

    mod prime_bag {
        use super::PrimeBag;
        use std::iter::FromIterator;

        pub fn from(ints: &[u64]) -> PrimeBag {
            PrimeBag::from(ints)
        }

        mod lowest_common_multiple {
            use super::*;

            #[test]
            fn merging_disjoint_prime_bags_leaves_us_with_both_sets_of_primes_merged() {
                let p = from(&vec![2, 3, 5]);
                let res = p.lowest_common_multiple(&from(&vec![7, 11, 13]));
                let result: Vec<u64> = Vec::from_iter(res);

                assert_eq!(result, vec![2, 3, 5, 7, 11, 13]);
            }

            #[test]
            fn merging_overlapping_prime_bags_dedups_where_possible() {
                let p = from(&vec![2, 3, 5]);
                let res = p.lowest_common_multiple(&from(&vec![3, 5, 7]));

                let result: Vec<u64> = Vec::from_iter(res);

                assert_eq!(result, vec![2, 3, 5, 7]);
            }

            #[test]
            fn lcm_of_prime_bags_with_same_factor_different_number_of_times_takes_the_max() {
                let p = from(&vec![2, 3, 5, 5]);
                println!("{:?}", p);
                let q = from(&vec![3, 3, 5, 7]);
                println!("{:?}", q);
                let res = p.lowest_common_multiple(&q);

                let result: Vec<u64> = Vec::from_iter(res);

                assert_eq!(result, vec![2, 3, 3, 5, 5, 7]);
            }
        }

        mod mulitplication {
            use super::*;

            #[test]
            fn multiply_two_prime_bags_combines_factors() {
                let p = from(&vec![2, 3, 5]);
                let q = from(&vec![3, 5, 7]);

                let result = p.multiply(&q);
                let result: Vec<u64> = Vec::from_iter(result);

                assert_eq!(result, vec![2, 3, 3, 5, 5, 7]);
            }
        }

        mod equality {
            use super::*;

            #[test]
            fn equality_between_primes() {
                let a = from(&[2]);
                let b = from(&[2]);
                let c = from(&[3]);

                assert!(a == a);
                assert!(a == b);
                assert!(a != c);
                assert!(b != c);
            }

            #[test]
            fn equality_between_non_primes() {
                let a = from(&[22]);
                let b = from(&[22]);
                let c = from(&[33]);

                assert!(a == a);
                assert!(a == b);
                assert!(a != c);
                assert!(b != c);
            }

            #[test]
            fn equality_between_different_expressions_of_same_number() {
                let a = from(&[2, 3, 5]);
                let b = from(&[2, 15]);
                let c = from(&[30]);

                assert!(a == a);
                assert!(a == b);
                assert!(a == c);
                assert!(b == c);
            }
        }

        mod division {
            use super::*;

            #[test]
            fn division_subtracts_factors() {
                let a = from(&[2, 3, 5]);
                let b = from(&[3]);

                let result = a.divide(&b);

                assert_eq!(result, from(&[2, 5]));
            }
        }

        mod ordering {
            use super::*;

            #[test]
            fn ordering_between_numbers_same_number_of_factors() {
                let a = from(&[2, 5]);
                let b = from(&[2, 2]);
                let c = from(&[2, 7]);

                assert!(a > b);
                assert!(a < c);
                assert!(b < c);
                assert!(c > a);
            }

            #[test]
            fn ordering_between_numbers_repeated_factors() {
                let a = from(&[2, 2, 5]);
                let b = from(&[2, 2, 7]);
                let c = from(&[2, 7, 7]);

                assert!(a < b);
                assert!(a < c);
                assert!(b < c);
                assert!(c > a);
                assert!(c > b);
                assert!(c >= c);
            }

            #[test]
            fn ordering_with_large_numb() {
                let a = from(&[1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 5, 5, 7, 7, 7]);
                let b = from(&[1, 1, 1, 2, 2, 3, 3, 3, 3, 3, 3, 5, 7, 7, 7]);
                let c = from(&[1, 1, 1, 2, 3, 3, 3, 3, 3, 3, 3, 5, 7, 7, 7]);

                assert!(a > b);
                assert!(c > b);
                assert!(b < c);
                assert!(a > c);
            }
        }

    }
}
