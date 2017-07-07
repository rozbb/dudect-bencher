// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Defines a `fn main()` that will run all benchmarks defined by listed functions `$function`
#[macro_export]
macro_rules! ctbench_main {
    ($($function:path),+) => {
        extern crate clap;
        use clap::App;
        use $crate::ctbench::{run_benches_console, BenchName, BenchNameAndFn, BenchOpts};
        fn main() {
            let mut benches = Vec::new();
            $(
                benches.push(BenchNameAndFn {
                    name: BenchName(stringify!($function)),
                    benchfn: $function,
                });
            )+
            let matches = App::new("dudect-bencher")
                .arg_from_usage("--filter [BENCH]\
                                 'Only run the benchmarks whose name contains BENCH'")
                .arg_from_usage("--continuous [BENCH]\
                                'Runs a continuous benchmark on the first bench matching BENCH'")
                .get_matches();

            let mut test_opts = BenchOpts::default();
            test_opts.filter = matches.value_of("continuous")
                                      .or(matches.value_of("filter"))
                                      .map(|s| s.to_string());
            test_opts.continuous = matches.is_present("continuous");

            run_benches_console(test_opts, benches).unwrap();
        }
    }
}
