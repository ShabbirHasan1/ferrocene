// Regression test for #51008 -- the anonymous lifetime in `&i32` was
// being incorrectly considered part of the "elided lifetimes" from
// the impl.
//
// run-pass

#![feature(rust_2018_preview)]

trait A {

}

impl<F> A for F where F: FnOnce(&i32) {}

fn main() {}

// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime Elision
//
// ferrocene-annotations: fls_hethxxbcg7ja
// Function Lifetime Elision
