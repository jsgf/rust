// --extern-location file: with a non-existent path

// edition:2018
// check-pass
// aux-crate:bar=bar.rs
// compile-flags:--extern-location bar=file:{{src-base}}/unused-crate-deps/file-does-not-exist.txt:123

#![warn(unused_crate_dependencies)]
//~^ WARNING external crate `bar` unused in

fn main() {}
