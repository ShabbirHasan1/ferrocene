// See https://github.com/rust-lang/rust/issues/88442
// run-rustfix
// edition:2018
// check-pass
#![allow(unused)]
#![warn(rust_2021_prelude_collisions)]

trait AnnotatableTryInto {
    fn try_into<T>(self) -> Result<T, Self::Error>
    where Self: std::convert::TryInto<T> {
        std::convert::TryInto::try_into(self)
    }
}

impl<T> AnnotatableTryInto for T where T: From<u8> {}

fn main() -> Result<(), &'static str> {
    let x: u64 = 1;
    x.try_into::<usize>().or(Err("foo"))?.checked_sub(1);
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~| WARNING this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!

    x.try_into::<usize>().or(Err("foo"))?;
    //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
    //~| WARNING this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!

    Ok(())
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
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
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
