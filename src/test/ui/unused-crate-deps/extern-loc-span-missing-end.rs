// --extern-location span: with no end

// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=span:file.txt:123

#![warn(unused_crate_dependencies)]

fn main() {}
