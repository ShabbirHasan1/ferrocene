// check-fail
// aux-build:some_crate.rs
// compile-flags:--extern some_other_crate
// edition:2018

fn main() {
    ::some_crate::hello(); //~ ERROR
}

// ferrocene-annotations: um_rustc_extern
