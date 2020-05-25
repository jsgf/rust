// --extern-location with a file/line reference

// check-pass
// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=file:{{src-base}}/unused-crate-deps/test.mk:300

#![warn(unused_crate_dependencies)]
//~^ WARNING external crate `bar` unused in

fn main() {}
