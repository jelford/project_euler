#![feature(test)]

extern crate project_euler;
extern crate test;


#[cfg(test)]
mod benches {

    use project_euler::primes::stream_of_primes;
    use test::Bencher;

    #[bench]
    fn small_seive_speed(b: &mut Bencher) {
        b.iter(|| {
            let x = stream_of_primes(1000);
            x.collect::<Vec<u64>>()
        })
    }

    #[bench]
    fn large_seive_speed(b: &mut Bencher) {
        b.iter(|| {
            let x = stream_of_primes(1000000);
            x.collect::<Vec<u64>>()
        })
    }
}
