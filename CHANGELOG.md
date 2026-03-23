# Changelog

Entries are listed in reverse chronological order.

## 0.7.0

* Bumped MSRV to 1.85
* Removed `core-hint-black-box` feature flag, since MSRV now covers `core::hint::black_box`
* Bumped `rand` to v0.10

## 0.6.0

* Added default `core-hint-black-box` feature, which enables Rust's built-in best-effort black box abstraction for versions >= 1.66.
* Fixed macro issue that required the end user to pull in `clap` as a dependency
