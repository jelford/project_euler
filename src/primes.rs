

struct StreamOfPrimes {
    seed_primes: Vec<u64>,
    current_index: usize,
    segment_size: usize,
    next_segment_floor: u64,
}

fn primes_from_next_seive_segment(floor: u64, size: usize, seed_primes: &Vec<u64>) -> Vec<u64> {
    let mut result = Vec::with_capacity(size / 2);

    let mut new_segment = vec![true; size+1];
    for i in seed_primes.iter() {
        let p = i.clone() as f64;

        let start_at = (floor as f64 / p).ceil() as u64;
        let end_at = ((floor as usize + size) as f64 / p).floor() as u64;

        for j in start_at..end_at+1 {
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
        } else {

            let mut next_segment = primes_from_next_seive_segment(self.next_segment_floor, self.segment_size, &self.seed_primes);

            self.seed_primes.append(&mut next_segment);

            self.next_segment_floor = self.next_segment_floor + self.segment_size as u64;

            self.next()
        }
    }
}



fn stream_of_primes(upper_bound: u64) -> StreamOfPrimes {
    let segment_size = (upper_bound as f64).sqrt().ceil() as usize;

    let mut initial_seive = vec![true; segment_size + 1];
    initial_seive[0] = false;
    initial_seive[1] = false;

    let mut seed_primes = Vec::with_capacity(segment_size / 2);

    for i in 2..segment_size {
        if initial_seive[i] {
            for j in i..(segment_size/i) {
                initial_seive[i*j] = false
            }
            seed_primes.push(i as u64);
        }
    }

    StreamOfPrimes {
        seed_primes: seed_primes,
        current_index: 0,
        segment_size: segment_size,
        next_segment_floor: segment_size as u64,
    }
}



pub fn highest_prime_factor(target: u64) -> u64 {
    if target == 1 {
        return 1;
    }

    let seive_size = (target as f64).sqrt().ceil() as u64;
    let mut seive = stream_of_primes(seive_size);

    let mut t = target;
    let mut factor : u64 = 2;

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
    use super::stream_of_primes;

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
}
