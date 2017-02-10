use std::ops::{Add, Mul, Div};

pub fn sum_of_i<T, R, Q>(n: T) -> Q::Output
    where T : Add<Output=R> + Copy + Mul<R, Output=Q> + From<u8>,
        Q : Div + From<u8>
    {
        ((n) * (n+T::from(1_u8))) / Q::from(2_u8)
}

pub fn sum_of_squares_i<T>(n: T) -> T
where T : Add<Output=T> + Copy + Mul<Output=T> + From<u8> + Div<Output=T>
{
    n * (n + T::from(1_u8)) * ((T::from(2) * n) + T::from(1)) / T::from(6)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn naive_sum_of_i(n: u64) -> u64 {
        let v = 1...n;
        v.sum()
    }

    fn naive_sum_of_squares(n: u64) -> u64 {
        let v = 1...n;
        v.map(|v| v*v).sum()
    }

    #[test]
    fn sum_of_i_base_cases() {
        assert_eq!(sum_of_i(1_u64), 1);
        assert_eq!(sum_of_i(2_u64), 3);
        assert_eq!(sum_of_i(3_u64), 6);
    }

    #[test]
    fn sum_of_i_is_correct_for_some_mid_size_numbers() {
        assert_eq!(sum_of_i(100_u64), naive_sum_of_i(100));
        assert_eq!(sum_of_i(200_u64), naive_sum_of_i(200));
    }

    #[test]
    fn sum_of_squares_base_cases() {
        assert_eq!(sum_of_squares_i(1), 1);
        assert_eq!(sum_of_squares_i(0), 0);
        assert_eq!(sum_of_squares_i(2), 5);
        assert_eq!(sum_of_squares_i(5), 55);
    }

    #[test]
    fn sum_of_squares_is_correct_for_some_mid_sized_numbers() {
        assert_eq!(sum_of_squares_i(100), naive_sum_of_squares(100));
        assert_eq!(sum_of_squares_i(200), naive_sum_of_squares(200));
    }
}
