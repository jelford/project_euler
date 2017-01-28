
use std::mem;

struct PalindromeStream {
    current_digits: Vec<i8>,
}

fn min_in_position(pos: usize, digits: usize) -> i8 {
    if pos == 0 && digits > 1 {
        1
    } else {
        0
    }
}

fn num_from_digits(digits: &Vec<i8>) -> u64 {
    println!("evaluating {:?}", digits);
    let mut num = 0_u64;
    for &d in digits.iter() {
        num *= 10;
        num += d as u64;
    }
    return num;
}

fn mirror<T: Clone>(v: &mut Vec<T>, pos: usize) {
    let mirror = v.len() - 1 - pos;

    v[mirror] = v[pos].clone();
}

fn step(digits: &Vec<i8>) -> Option<Vec<i8>> {
    let mut res = digits.clone();
    let mut target = 0;
    let len = res.len();

    let limit = (len / 2) + (if len % 2 == 1 { 1 } else { 0 }) - 1;

    while res[target] == min_in_position(target, res.len()) {
        target += 1;
        if target > limit {
            return None;
        }
        for i in 0..target {
            res[i] = 9;
            mirror(&mut res, i);
        }
    }
    res[target] -= 1;
    mirror(&mut res, target);

    return Some(res)
}

impl Iterator for PalindromeStream {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(updated) = step(&self.current_digits) {
            mem::replace(&mut self.current_digits, updated);
            Some(num_from_digits(&self.current_digits))
        } else {
            None
        }
    }
}

fn palindromes_with_number_of_digits(digits: usize) -> PalindromeStream {
    let mut res = PalindromeStream {
        current_digits: vec![9; digits],
    };
    res.current_digits[0] += 1;
    res
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use super::palindromes_with_number_of_digits;
    use super::num_from_digits;
    use super::step;
    use super::mirror;

    #[test]
    fn mirror_is_a_noop_for_vec_of_1_lenght() {
        let mut target = vec![5];
        mirror(&mut target, 0); assert_eq!(target, vec![5]);
    }

    #[test]
    fn mirror_duplicates_first_element_for_vec_len_2() {
        let mut target = vec![4, 6];
        mirror(&mut target, 0); assert_eq!(target, vec![4, 4]);
    }

    #[test]
    fn mirror_is_a_noop_for_mid_point() {
        let mut target = vec![4, 6, 73];
        mirror(&mut target, 1); assert_eq!(target, vec![4, 6, 73]);
    }

    #[test]
    fn mirror_dupes_elements_with_offset_in_array() {
        let mut target = vec![4, 6, 73, 5];
        mirror(&mut target, 1); assert_eq!(target, vec![4, 6, 6, 5]);
    }

    #[test]
    fn step_on_single_digit_decrements() {
        let mut target = vec![9];
        for _ in 0..8 {
            let old = target[0];
            target = step(&target).unwrap();
            assert_eq!(target[0], old-1);
        }
    }

    #[test]
    fn step_reduces_both_numbers_if_two_digits() {
        assert_eq!(step(&vec![9, 9]), Some(vec![8, 8]));
        assert_eq!(step(&vec![8, 8]), Some(vec![7, 7]));
        assert_eq!(step(&vec![7, 7]), Some(vec![6, 6]));
        assert_eq!(step(&vec![2, 2]), Some(vec![1, 1]));
    }

    #[test]
    fn step_reduces_outside_numbers_of_3_digits() {
        assert_eq!(step(&vec![9, 9, 9]), Some(vec![8, 9, 8]));
        assert_eq!(step(&vec![8, 9, 8]), Some(vec![7, 9, 7]));
        assert_eq!(step(&vec![7, 9, 7]), Some(vec![6, 9, 6]));
        assert_eq!(step(&vec![2, 9, 2]), Some(vec![1, 9, 1]));
    }

    #[test]
    fn step_reduces_middle_number_of_three_digits_when_outside_exhausted() {
        assert_eq!(step(&vec![1, 9, 1]), Some(vec![9,8,9]));
    }

    #[test]
    fn step_says_its_done_when_reaches_lowest_point() {
        assert_eq!(step(&vec![0]), None);
        assert_eq!(step(&vec![1, 1]), None);
        assert_eq!(step(&vec![1, 0, 1]), None);
    }

    #[test]
    fn num_from_digits_works_with_decimals() {
        assert_eq!(num_from_digits(&vec![1]), 1_u64);
        assert_eq!(num_from_digits(&vec![9, 9]), 99_u64);
        assert_eq!(num_from_digits(&vec![1, 2, 3]), 123_u64);
    }

    #[test]
    fn num_from_digits_is_zero_for_empty_digits() {
        assert_eq!(num_from_digits(&vec![]), 0);
    }

    #[test]
    fn all_1_digit_numbers_are_palindromes() {
        let mut r: Vec<u64> = palindromes_with_number_of_digits(1).take(100).collect();
        r.sort();
        assert_eq!(r, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn all_multiples_of_11_are_2_digit_palindromes() {
        let ps = palindromes_with_number_of_digits(2).take(100);
        let r: HashSet<u64> = HashSet::from_iter(ps);

        for i in 1..9 {
            assert_is_palindrome(i*11, &r);
        }
    }

    fn assert_is_palindrome(target: u64, palindromes: &HashSet<u64>) {
        assert!(palindromes.contains(&target),
                "{} not considered palindromic",
                target);
    }

    #[test]
    fn can_generate_3_digit_palindromes() {
        let ps = palindromes_with_number_of_digits(3).take(100);
        let r: HashSet<u64> = HashSet::from_iter(ps);

        assert_is_palindrome(101, &r);
        assert_is_palindrome(909, &r);
        assert_is_palindrome(333, &r);
        assert_is_palindrome(181, &r);
        assert_is_palindrome(939, &r);
        assert_is_palindrome(888, &r);
    }

    #[test]
    fn covers_all_length_3_palindromes() {
        let r = palindromes_with_number_of_digits(3).collect::<Vec<_>>();
        assert_eq!(r.len(), 90);
    }

    #[test]
    fn covers_all_length_5_palindromes() {
        let r = palindromes_with_number_of_digits(5).collect::<Vec<_>>();
        assert_eq!(r.len(), 900);
    }
}
