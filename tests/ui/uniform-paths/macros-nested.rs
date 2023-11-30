// This test is similar to `macros.rs`, but nested in modules.

// run-pass
// edition:2018

#![allow(non_camel_case_types)]

mod foo {
    // Test that ambiguity errors are not emitted between `self::test` and
    // `::test`, assuming the latter (crate) is not in `extern_prelude`.
    macro_rules! m1 {
        () => {
            mod test {
                pub struct Foo(pub ());
            }
        }
    }
    pub use test::Foo;
    m1!();

    // Test that qualified paths can refer to both the external crate and local item.
    macro_rules! m2 {
        () => {
            mod std {
                pub struct io(pub ());
            }
        }
    }
    pub use ::std::io as std_io;
    pub use self::std::io as local_io;
    m2!();
}

// Test that we can refer to the external crate unqualified
// (when there isn't a local item with the same name).
use std::io;

mod bar {
    // Also test the unqualified external crate import in a nested module,
    // to show that the above import doesn't resolve through a local `std`
    // item, e.g., the automatically injected `extern crate std;`, which in
    // the Rust 2018 should no longer be visible through `crate::std`.
    pub use std::io;
}


fn main() {
    foo::Foo(());
    let _ = foo::std_io::stdout();
    foo::local_io(());
    let _ = io::stdout();
    let _ = bar::io::stdout();
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
//
// ferrocene-annotations: fls_151r19d7xbgz
// Entities
//
// ferrocene-annotations: fls_izl8iuhoz9e0
// Scopes
//
// ferrocene-annotations: fls_6ozthochxz1i
// Binding Scopes
//
// ferrocene-annotations: fls_ftphlagzd2te
// Generic Parameter Scope
//
// ferrocene-annotations: fls_m0z7omni9hp0
// Item Scope
//
// ferrocene-annotations: fls_769b4p8v3cwu
// Label Scope
//
// ferrocene-annotations: fls_kgbi26212eof
// Self Scope
//
// ferrocene-annotations: fls_octf6sf7yso
// Textual Macro Scope
//
// ferrocene-annotations: fls_lnpyb285qdiy
// Scope Hierarchy
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_40xoego2thsp
// Resolution
//
// ferrocene-annotations: fls_i6qzga6dyaee
// Path Resolution
//
// ferrocene-annotations: fls_bbso3c45kr9z
// Simple Path Resolution
//
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
