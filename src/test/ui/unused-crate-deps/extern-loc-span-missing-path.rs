// --extern-location span: with no path

// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=span:

#![warn(unused_crate_dependencies)]

fn main() {}
