#![cfg_attr(rustfmt, rustfmt_skip)] // disable rustfmt for this file

pub enum Times { Two, Four }

pub fn slice_times(slice: &mut [f64], times: Times) {
    match times {
        Times::Two => slice_times_two(slice),
        Times::Four => slice_times_four(slice),
    }
}

///// ALL BELOW THIS POINT IS PRIVATE /////

// Functions we want to test, but unfortunately they are private
fn slice_times_two(v: &mut [f64]) {
    for x in v { *x *= 2.0 }
}

fn slice_times_four(v: &mut [f64]) {
    for x in v { *x *= 4.0 }
}

#[cfg(feature = "bench")]
mod benches {

    use diol::prelude::*;

    use crate::{slice_times_two, slice_times_four};

    #[test] // benches can be run as tests
    fn run_all_benches() {
        let mut bench = Bench::new(BenchConfig::default());
        
        bench.register(bench_slice_times_two, [1, 4, 16, 128]);
        bench.register(bench_slice_times_four, [1, 4, 16, 128]);

        bench.run().expect("Unable to run benchmarks");
    }

    // Those functions are like bench-config boilerplate.
    // I think having one of this for each thing we
    // want to test is acceptable.
    fn bench_slice_times_two(bencher: Bencher, len: usize) {
        let mut v = vec![0.0f64; len];
        bencher.bench(|| {
            slice_times_two(&mut v);
            black_box(&mut v);
        });
    }

    fn bench_slice_times_four(bencher: Bencher, len: usize) {
        let mut v = vec![0.0f64; len];
        bencher.bench(|| {
            slice_times_four(&mut v);
            black_box(&mut v);
        });
    }
}

#[cfg(test)]
mod tests {

    use crate::{slice_times_two, slice_times_four};

    #[test]
    fn something_works() {
        let mut vec = vec![0.0; 15];

        slice_times_two(&mut vec);
        slice_times_four(&mut vec);

        assert!(!vec.is_empty());
    }
}
