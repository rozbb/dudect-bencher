// Copyright 2012-2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate implements the [DudeCT](https://eprint.iacr.org/2016/1123) statistical methods for
//! testing constant-time functions. It is based loosely off of the
//! [`bencher`](https://github.com/bluss/bencher) crate.
//!
//! The core idea of the DudeCT test is to construct two sets of inputs for the function in
//! question `f`. The two sets of inputs will correspond to the `Left` distribution and the `Right`
//! distribution. The user is expected to select inputs that demonstrate the potential time
//! discrepency that they are trying to test.
//!
//! For example, if `f` tests the equality of two given vectors, the `Left` distribution might
//! contain random pairs of equal vectors, while the `Right` distribution might contain vectors who
//! differ at random places or maybe in a fixed place. Once these two input sets are established,
//! the test is performed and the statistical difference of the distribution of runtimes is
//! calculated. If the function behaves significantly differently for inputs from `Left` vs
//! `Right`, the runtime distributions should be significantly different. This difference will be
//! reflected in the computed t value. See `examples/ctbench-foo.rs` for example code
//!
//! The program output looks like
//!
//! ```text
//! bench array_eq ... : n == +0.046M, max t = +61.61472, max tau = +0.28863, (5/tau)^2 = 300
//! ```
//!
//! It is interpreted as follows. Firstly note that the runtime distributions are cropped at
//! different percentiles and about 100 t-tests are performed. Of these t-tests, the one that
//! produces the largest absolute t-value is printed as `max_t`. The other values printed are
//!
//!  * `n`, indicating the number of samples used in computing this t-value
//!  * `max_tau`, which is the t-value scaled for the samples size (formally, `max_tau = max_t /
//!    sqrt(n)`)
//!  * `(5/tau)^2`, which indicates the number of measurements that would be needed to distinguish
//!    the two distributions with t > 5
//!
//! t-values greater than 5 are generally considered a good indication that the function is not
//! constant time. t-values less than 5 does not necessarily imply that the function is
//! constant-time, since there may be other input distributions under which the function behaves
//! significantly differently.

// TODO: More comments
// TODO: Do "higher order preprocessing" from the paper

extern crate clap;
extern crate ctrlc;
extern crate rand;
extern crate rand_chacha;

pub mod ctbench;
mod stats;
mod toplevel;

pub use ctbench::{BenchRng, Class, CtRunner};
