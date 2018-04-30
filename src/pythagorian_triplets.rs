fn is_pythagorian_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

#[derive(Clone, Debug)]
pub struct PythagPoint {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}

pub struct PythagorianPointIterator {
    current: PythagPoint,
}

pub fn pythagorian_triplet_stream() -> PythagorianPointIterator {
    PythagorianPointIterator {
        current: PythagPoint { a: 0, b: 0, c: 5 },
    }
}

impl Iterator for PythagorianPointIterator {
    type Item = PythagPoint;
    fn next(&mut self) -> Option<PythagPoint> {
        let c = self.current.c;

        'outer: for c in c.. {
            for i in self.current.a + 1..c {
                for j in self.current.b..c {
                    if i.pow(2) + j.pow(2) > c.pow(2) {
                        self.current.b = i;
                        break;
                    }

                    if is_pythagorian_triplet(i, j, c) {
                        self.current = PythagPoint { a: i, b: j, c: c };
                        break 'outer;
                    }
                }
            }

            self.current.a = 1;
            self.current.b = 1;
        }

        Some(self.current.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn some_pythagorian_triplets() {
        assert!(is_pythagorian_triplet(3, 4, 5));
        for i in 0..10 {
            assert!(is_pythagorian_triplet(i * 3, i * 4, i * 5));
        }
    }

    #[test]
    fn some_non_examples_of_pythagorian_triplets() {
        assert!(!is_pythagorian_triplet(1, 2, 3));
        assert!(!is_pythagorian_triplet(2, 2, 2));
        assert!(!is_pythagorian_triplet(4, 5, 6));
        assert!(!is_pythagorian_triplet(13, 26, 12));
        assert!(!is_pythagorian_triplet(5, 6, 8));
        assert!(!is_pythagorian_triplet(126, 127, 201));
    }

    #[test]
    fn all_returned_pythagpoints_are_pythagorian_triplets() {
        let mut stream = pythagorian_triplet_stream();
        let mut last: Option<PythagPoint> = None;
        for _ in 0..30 {
            let n = stream.next().unwrap();
            assert!(is_pythagorian_triplet(n.a, n.b, n.c));

            // Assert that progress is being made.
            match last {
                None => {}
                Some(p) => {
                    assert!(p.a < n.a || p.b < n.b || p.c < n.c);
                }
            }
            last = Some(n.clone());
        }
    }
}
