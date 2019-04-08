#[macro_use]
extern crate dudect_bencher;
extern crate rand;

use dudect_bencher::{BenchRng, Class, CtRunner};
use rand::prelude::*;

// Return a random vector of length len
fn rand_vec(len: usize, rng: &mut BenchRng) -> Vec<u8> {
    let mut arr = vec![0u8; len];
    rng.fill_bytes(&mut arr);
    arr
}

// Benchmark for some random arithmetic operations. This should produce small t-values
fn arith(runner: &mut CtRunner, rng: &mut BenchRng) {
    let mut inputs = Vec::new();
    let mut classes = Vec::new();

    // Make 100,000 inputs on each run
    for _ in 0..100_000 {
        inputs.push(rng.gen::<usize>());
        // Randomly pick which distribution this example belongs to
        if rng.gen::<bool>() {
            classes.push(Class::Left);
        }
        else {
            classes.push(Class::Right);
        }
    }

    for (u, class) in inputs.into_iter().zip(classes.into_iter()) {
        // Time some random arithmetic operations
        runner.run_one(class, || ((u + 10) / 6) << 5);
    }
}

// Benchmark for equality of vectors. This does an early return when it finds an inequality, so it
// should be very much not constant-time
fn vec_eq(runner: &mut CtRunner, rng: &mut BenchRng) {
    // Make vectors of size 100
    let vlen = 100;
    let mut inputs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    let mut classes = Vec::new();

    // Make 100,000 random pairs of vectors
    for _ in 0..100_000 {
        // Flip a coin. If true, make a pair of vectors that are equal to each other and put it
        // in the Left distribution
        if rng.gen::<bool>() {
            let v1 = rand_vec(vlen, rng);
            let v2 = v1.clone();
            inputs.push((v1, v2));
            classes.push(Class::Left);
        }
        // Otherwise, make a pair of vectors that differ at the 6th element and put it in the
        // right distribution
        else {
            let v1 = rand_vec(vlen, rng);
            let mut v2 = v1.clone();
            v2[5] = 7;
            inputs.push((v1, v2));
            classes.push(Class::Right);
        }
    }

    for (class, (u, v)) in classes.into_iter().zip(inputs.into_iter()) {
        // Now time how long it takes to do a vector comparison
        runner.run_one(class, || u == v);
    }
}

// Expand the main function to include benches for arith and vec_eq
ctbench_main_with_seeds!(
    (arith, Some(0x6b6c816d)),
    (vec_eq, None)
);
// Alternatively, for no explicit seeds, you can use
// ctbench_main!(arith, vec_eq);
