// run-pass
// compile-flags: -Coverflow-checks=y -Coverflow-checks=n

// Rightmost flag overrides any previous flags.

#[allow(arithmetic_overflow)]
fn main() {
    let x: i8 = 100;
    let y: i8 = 100;
    let z: i8 = x + y;
    assert_eq!(z, -56);
}

// ferrocene-annotations: um_rustc_C_overflow_checks
