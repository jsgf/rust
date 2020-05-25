// --extern-location span: with no start

// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=span:file.txt

#![warn(unused_crate_dependencies)]

fn main() {}
