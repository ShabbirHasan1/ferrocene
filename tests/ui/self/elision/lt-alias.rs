// check-pass

#![allow(non_snake_case)]

use std::rc::Rc;

struct Struct<'a> { x: &'a u32 }

type Alias<'a> = Struct<'a>;

impl<'a> Alias<'a> {
    fn take_self(self, f: &u32) -> &u32 {
        f
    }

    fn take_Alias(self: Alias<'a>, f: &u32) -> &u32 {
        f
    }

    fn take_Box_Alias(self: Box<Alias<'a>>, f: &u32) -> &u32 {
        f
    }

    fn take_Box_Box_Alias(self: Box<Box<Alias<'a>>>, f: &u32) -> &u32 {
        f
    }

    fn take_Rc_Alias(self: Rc<Alias<'a>>, f: &u32) -> &u32 {
        f
    }

    fn take_Box_Rc_Alias(self: Box<Rc<Alias<'a>>>, f: &u32) -> &u32 {
        f
    }
}

fn main() { }

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
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
