// --extern-location file: with no path

// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=file:

#![warn(unused_crate_dependencies)]

fn main() {}
