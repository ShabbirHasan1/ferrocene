fn foo(x: &mut u32) {
    let bar = || { foo(x); };
    bar(); //~ ERROR cannot borrow
}
fn main() {}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
