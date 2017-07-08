#[macro_use]
extern crate dudect_bencher;
extern crate rand;

use dudect_bencher::{Class, CtBencher};
use rand::Rng;

// Return a random vector of length len
fn rand_vec(len: usize) -> Vec<u8> {
    let mut csprng = rand::OsRng::new().unwrap();
    let mut arr = vec![0u8; len];
    csprng.fill_bytes(&mut arr);
    arr
}

// Benchmark for some random arithmetic operations. This should produce small t-values
fn arith(bench: &mut CtBencher) {
    // The inside of .iter() will run indefinitely in continuous mode. Otherwise it'll run once
    bench.iter(|runner| {
        let mut csprng = rand::OsRng::new().unwrap();
        let mut inputs = Vec::new();
        let mut classes = Vec::new();

        // Make 20,000 inputs on each run
        for _ in 0..100_000 {
            inputs.push(csprng.gen::<usize>());
            // Randomly pick which distribution this example belongs to
            if csprng.gen::<bool>() {
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
    });
}

// Benchmark for equality of vectors. This does an early return when it finds an inequality, so it
// should be very much not constant-time
fn vec_eq(bench: &mut CtBencher) {
    // Make vectors of size 100
    let vlen = 100;
    bench.iter(|ref mut runner| {
        let mut csprng = rand::OsRng::new().unwrap();
        let mut inputs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
        let mut classes = Vec::new();

        // Make 50,000 random pairs of vectors
        for _ in 0..100_000 {
            // Flip a coin. If true, make a pair of vectors that are equal to each other and put it
            // in the Left distribution
            if csprng.gen::<bool>() {
                let v1 = rand_vec(vlen);
                let v2 = v1.clone();
                inputs.push((v1, v2));
                classes.push(Class::Left);
            }
            // Otherwise, make a pair of vectors that differ at the 6th element and put it in the
            // right distribution
            else {
                let v1 = rand_vec(vlen);
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
    });
}

// Expand the main function to include benches for arith and vec_eq
ctbench_main!(arith, vec_eq);
