// --extern-location file: with no line number

// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=file:file.txt:123:abc

#![warn(unused_crate_dependencies)]

fn main() {}
