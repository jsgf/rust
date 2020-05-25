// --extern-location file: with no line number

// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=file:file.txt

#![warn(unused_crate_dependencies)]

fn main() {}
