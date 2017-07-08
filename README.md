# dudect-bencher
[![Version](https://img.shields.io/crates/v/dudect-bencher.svg)](https://crates.io/crates/dudect-bencher)
[![Docs](https://docs.rs/dudect-bencher/badge.svg)](https://docs.rs/dudect-bencher)

This crate implements the [DudeCT](https://eprint.iacr.org/2016/1123.pdf) statistical methods for
testing constant-time functions. It is based loosely off of the
[`bencher`](https://github.com/bluss/bencher) crate.

# Usage

Example use is as follows. Since this requires the current crate as a dependency, it is easiest to
put the benchmarks in `examples/`. Take a look at `examples/ctbench-foo.rs` for sample source code.

To run all the benchmarks in `examples/ctbench-foo.rs`, you can simply run `cargo run --release
--example ctbench-foo`.

To run a subset of the benchmarks in the above file that have a the substring `ar` in it, run
`cargo run --release --example ctbench-foo -- --filter ar`.

To run the `vec_eq` benchmark continuously, collecting more samples as it goes along, run `cargo run
--release --example ctbench-foo -- --continuous vec_eq`.

To run the benchmarks in `ctbench-foo` and get the raw runtimes in CSV format, run `cargo run
--release --example ctbench-foo -- --out data.csv`.

# Interpreting Output

The benchmark output looks like

```
bench array_eq ... : n == +0.046M, max t = +61.61472, max tau = +0.28863, (5/tau)^2 = 300
```

It is interpreted as follows. Firstly note that the runtime distributions are cropped at different
percentiles and about 100 t-tests are performed. Of these t-tests, the one that produces the largest
absolute t-value is printed as `max_t`. The other values printed are

 * `n`, indicating the number of samples used in computing this t-value
 * `max_tau`, which is the t-value scaled for the samples size (formally, `max_tau = max_t /
   sqrt(n)`)
 * `(5/tau)^2`, which indicates the number of measurements that would be needed to distinguish the
   two distributions with t > 5

t-values greater than 5 are generally considered a good indication that the function is not constant
time. t-values less than 5 does not necessarily imply that the function is constant-time, since
there may be other input distributions under which the function behaves significantly differently.

# License

Again, this project derives from the [`bencher`](https://github.com/bluss/bencher) crate under the
MIT license. This project is licensed under the ([MIT license](LICENSE-MIT)) as well.
