// --extern-location with a file/span reference

// edition:2018
// check-pass
// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=span:{{src-base}}/unused-crate-deps/test.mk:45:53

#![warn(unused_crate_dependencies)]
//~^ WARNING external crate `bar` unused in

fn main() {}
