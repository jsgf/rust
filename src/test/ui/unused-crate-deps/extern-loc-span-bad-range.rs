// --extern-location span with bad range

// check-pass
// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=span:{{src-base}}/unused-crate-deps/test.mk:300000:400000

#![warn(unused_crate_dependencies)]
//~^ WARNING external crate `bar` unused in

fn main() {}
