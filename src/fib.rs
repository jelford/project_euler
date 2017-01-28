
pub struct FibonacciSeries {
    prev_1 : u64,
    prev_0 : u64,
}

pub fn fib_series() -> FibonacciSeries {
    FibonacciSeries { prev_1: 0, prev_0: 1 }
}

impl Iterator for FibonacciSeries {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.prev_1 + self.prev_0;
        self.prev_1 = self.prev_0;
        self.prev_0 = res;
        Some(res)
    }
}


#[cfg(test)]
mod tests {
    use super::fib_series;
    use std::option::Option::Some;

    #[test]
    fn fibcalc_yields_fib_series() {
        let mut fc = fib_series();
        assert_eq!(fc.next(), Some(1));
        assert_eq!(fc.next(), Some(2));
        assert_eq!(fc.next(), Some(3));
        assert_eq!(fc.next(), Some(5));
        assert_eq!(fc.next(), Some(8));
        assert_eq!(fc.next(), Some(13));
    }
}
