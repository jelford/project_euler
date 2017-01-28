use std::iter::FromIterator;

pub struct StreamOfPrimes {
    seed_primes: Vec<u64>,
    upper_bound: u64,
    current_index: usize,
    segment_size: usize,
    next_segment_floor: u64,
}

fn primes_from_next_seive_segment(floor: u64, size: usize, seed_primes: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::with_capacity(size / 2);

    let mut new_segment = vec![true; size+1];
    for i in seed_primes.iter().take_while(|&x| x.pow(2) < (floor + size as u64)) {

        let p = i.clone() as f64;
        let start_at = (floor as f64 / p).ceil() as u64;
        let end_at = start_at + (size as f64 / p).floor() as u64;

        for j in start_at..end_at {
            let target = (i * j) - floor;
            new_segment[target as usize] = false;
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
            let mut next_segment = primes_from_next_seive_segment(self.next_segment_floor,
                                                                  self.segment_size,
                                                                  &self.seed_primes);

            self.seed_primes.append(&mut next_segment);

            self.next_segment_floor = self.next_segment_floor + self.segment_size as u64;

            self.next()
        }
    }
}

pub fn stream_of_primes(upper_bound: u64) -> StreamOfPrimes {

    let segment_size = if upper_bound > 1000 {
        (upper_bound as f64).sqrt().ceil() as usize
    } else {
        (upper_bound + 1) as usize
    };

    let mut initial_seive = vec![true; segment_size + 1];
    initial_seive[0] = false;
    initial_seive[1] = false;

    let mut seed_primes = Vec::with_capacity(segment_size / 2);

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
    pub fn merged_with(&self, other: &PrimeBag) -> PrimeBag {
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

    pub fn from<T>(from: T) -> PrimeBag
        where T: IntoIterator<Item = u64>
    {
        PrimeBag { primes: Vec::from_iter(from) }
    }

    pub fn empty() -> PrimeBag {
        PrimeBag { primes: vec![] }
    }
}

impl IntoIterator for PrimeBag {
    type Item = u64;
    type IntoIter = ::std::vec::IntoIter<u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.primes.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::stream_of_primes;
    use super::PrimeBag;
    use std::iter::FromIterator;

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
        assert_eq!(r,
                   vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67,
                        71, 73, 79, 83, 89, 97]);
    }

    #[test]
    fn merging_disjoint_prime_bags_leaves_us_with_both_sets_of_primes_merged() {
        let p = PrimeBag::from(vec![2, 3, 5]);
        let res = p.merged_with(&PrimeBag::from(vec![7, 11, 13]));
        let result: Vec<u64> = Vec::from_iter(res);

        assert_eq!(result, vec![2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn merging_overlapping_prime_bags_dedups_where_possible() {
        let p = PrimeBag::from(vec![2, 3, 5]);
        let res = p.merged_with(&PrimeBag::from(vec![3, 5, 7]));

        let result: Vec<u64> = Vec::from_iter(res);

        assert_eq!(result, vec![2, 3, 5, 7]);
    }

    #[test]
    fn merging_prime_bags_with_same_factor_different_number_of_times_takes_the_max() {
        let p = PrimeBag::from(vec![2, 3, 5, 5]);
        let res = p.merged_with(&PrimeBag::from(vec![3, 3, 5, 7]));

        let result: Vec<u64> = Vec::from_iter(res);

        assert_eq!(result, vec![2, 3, 3, 5, 5, 7]);
    }
}
