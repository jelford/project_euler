

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
        let end_at = ((floor as usize + size) as f64 / p).floor() as u64;

        for j in start_at..end_at + 1 {
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





#[cfg(test)]
mod tests {
    use super::stream_of_primes;


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
}
