// run-pass
#![allow(non_upper_case_globals)]

// pretty-expanded FIXME #23616

trait TheTrait { fn dummy(&self) { } }

impl TheTrait for &'static isize { }

fn foo<'a,T>(_: &'a T) where &'a T : TheTrait { }

fn bar<T>(_: &'static T) where &'static T : TheTrait { }

fn main() {
    static x: isize = 1;
    foo(&x);
    bar(&x);
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
