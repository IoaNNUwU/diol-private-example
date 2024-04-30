### Example of using `diol` crate to test private functions.
---
### > cargo test

```
running 1 test
test tests::something_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Runs 1 test `something_works`. See `lib.rs::tests` module.
- Doesn't run benchmarks
---
### > cargo test --features bench

```
running 2 tests
test tests::something_works ... ok
╭───────────────────────┬──────┬───────────┬───────────┬───────────┬───────────╮
│ benchmark             │ args │   fastest │    median │      mean │    stddev │
├───────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_two │    1 │  25.50 ns │  27.02 ns │  27.41 ns │   2.24 ns │
├───────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_two │    4 │  44.34 ns │  47.00 ns │  47.56 ns │   2.62 ns │
├───────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_two │   16 │ 100.82 ns │ 126.02 ns │ 127.08 ns │   5.13 ns │
├───────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_two │  128 │ 691.31 ns │ 840.94 ns │ 812.98 ns │  71.20 ns │
╰───────────────────────┴──────┴───────────┴───────────┴───────────┴───────────╯
╭────────────────────────┬──────┬───────────┬───────────┬───────────┬───────────╮
│ benchmark              │ args │   fastest │    median │      mean │    stddev │
├────────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_four │    1 │  25.75 ns │  27.02 ns │  27.41 ns │   1.94 ns │
├────────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_four │    4 │  43.79 ns │  46.92 ns │  47.45 ns │   2.75 ns │
├────────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_four │   16 │ 105.67 ns │ 125.47 ns │ 125.97 ns │   6.18 ns │
├────────────────────────┼──────┼───────────┼───────────┼───────────┼───────────┤
│ bench_slice_times_four │  128 │ 692.56 ns │ 856.00 ns │ 844.21 ns │  52.54 ns │
╰────────────────────────┴──────┴───────────┴───────────┴───────────┴───────────╯
test benches::run_all_benches ... ok
```

Runs test `something_works` as well as benchmarks in `lib.rs::benches` module.
- Runs all tests, including those which aren't benches, but I think it is acceptable.

To make everything work as it should, each test module should be given another feature-flag, but then to run tests you would do `cargo test --features testing`, which is annoying.
