// error-pattern: can't capture dynamic environment in a fn item
fn foo() {
    let x: isize;
    fn bar() { log(debug, x); }
}
fn main() { foo(); }

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
