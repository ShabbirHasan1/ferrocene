// run-rustfix

// The `try_into` suggestion doesn't include this, but we do suggest it after applying it
use std::convert::TryInto;

fn foo<N>(_x: N) {}

fn main() {
    let x_usize: usize = 1;
    let x_u64: u64 = 2;
    let x_u32: u32 = 3;
    let x_u16: u16 = 4;
    let x_u8: u8 = 5;
    let x_isize: isize = 6;
    let x_i64: i64 = 7;
    let x_i32: i32 = 8;
    let x_i16: i16 = 9;
    let x_i8: i8 = 10;
    let x_f64: f64 = 11.0;
    let x_f32: f32 = 12.0;

    foo::<usize>(x_usize);
    foo::<usize>(x_u64);
    //~^ ERROR mismatched types
    foo::<usize>(x_u32);
    //~^ ERROR mismatched types
    foo::<usize>(x_u16);
    //~^ ERROR mismatched types
    foo::<usize>(x_u8);
    //~^ ERROR mismatched types
    foo::<usize>(x_isize);
    //~^ ERROR mismatched types
    foo::<usize>(x_i64);
    //~^ ERROR mismatched types
    foo::<usize>(x_i32);
    //~^ ERROR mismatched types
    foo::<usize>(x_i16);
    //~^ ERROR mismatched types
    foo::<usize>(x_i8);
    //~^ ERROR mismatched types
    // foo::<usize>(x_f64);
    // foo::<usize>(x_f32);

    foo::<isize>(x_usize);
    //~^ ERROR mismatched types
    foo::<isize>(x_u64);
    //~^ ERROR mismatched types
    foo::<isize>(x_u32);
    //~^ ERROR mismatched types
    foo::<isize>(x_u16);
    //~^ ERROR mismatched types
    foo::<isize>(x_u8);
    //~^ ERROR mismatched types
    foo::<isize>(x_isize);
    foo::<isize>(x_i64);
    //~^ ERROR mismatched types
    foo::<isize>(x_i32);
    //~^ ERROR mismatched types
    foo::<isize>(x_i16);
    //~^ ERROR mismatched types
    foo::<isize>(x_i8);
    //~^ ERROR mismatched types
    // foo::<isize>(x_f64);
    // foo::<isize>(x_f32);

    foo::<u64>(x_usize);
    //~^ ERROR mismatched types
    foo::<u64>(x_u64);
    foo::<u64>(x_u32);
    //~^ ERROR mismatched types
    foo::<u64>(x_u16);
    //~^ ERROR mismatched types
    foo::<u64>(x_u8);
    //~^ ERROR mismatched types
    foo::<u64>(x_isize);
    //~^ ERROR mismatched types
    foo::<u64>(x_i64);
    //~^ ERROR mismatched types
    foo::<u64>(x_i32);
    //~^ ERROR mismatched types
    foo::<u64>(x_i16);
    //~^ ERROR mismatched types
    foo::<u64>(x_i8);
    //~^ ERROR mismatched types
    // foo::<u64>(x_f64);
    // foo::<u64>(x_f32);

    foo::<i64>(x_usize);
    //~^ ERROR mismatched types
    foo::<i64>(x_u64);
    //~^ ERROR mismatched types
    foo::<i64>(x_u32);
    //~^ ERROR mismatched types
    foo::<i64>(x_u16);
    //~^ ERROR mismatched types
    foo::<i64>(x_u8);
    //~^ ERROR mismatched types
    foo::<i64>(x_isize);
    //~^ ERROR mismatched types
    foo::<i64>(x_i64);
    foo::<i64>(x_i32);
    //~^ ERROR mismatched types
    foo::<i64>(x_i16);
    //~^ ERROR mismatched types
    foo::<i64>(x_i8);
    //~^ ERROR mismatched types
    // foo::<i64>(x_f64);
    // foo::<i64>(x_f32);

    foo::<u32>(x_usize);
    //~^ ERROR mismatched types
    foo::<u32>(x_u64);
    //~^ ERROR mismatched types
    foo::<u32>(x_u32);
    foo::<u32>(x_u16);
    //~^ ERROR mismatched types
    foo::<u32>(x_u8);
    //~^ ERROR mismatched types
    foo::<u32>(x_isize);
    //~^ ERROR mismatched types
    foo::<u32>(x_i64);
    //~^ ERROR mismatched types
    foo::<u32>(x_i32);
    //~^ ERROR mismatched types
    foo::<u32>(x_i16);
    //~^ ERROR mismatched types
    foo::<u32>(x_i8);
    //~^ ERROR mismatched types
    // foo::<u32>(x_f64);
    // foo::<u32>(x_f32);

    foo::<i32>(x_usize);
    //~^ ERROR mismatched types
    foo::<i32>(x_u64);
    //~^ ERROR mismatched types
    foo::<i32>(x_u32);
    //~^ ERROR mismatched types
    foo::<i32>(x_u16);
    //~^ ERROR mismatched types
    foo::<i32>(x_u8);
    //~^ ERROR mismatched types
    foo::<i32>(x_isize);
    //~^ ERROR mismatched types
    foo::<i32>(x_i64);
    //~^ ERROR mismatched types
    foo::<i32>(x_i32);
    foo::<i32>(x_i16);
    //~^ ERROR mismatched types
    foo::<i32>(x_i8);
    //~^ ERROR mismatched types
    // foo::<i32>(x_f64);
    // foo::<i32>(x_f32);

    foo::<u16>(x_usize);
    //~^ ERROR mismatched types
    foo::<u16>(x_u64);
    //~^ ERROR mismatched types
    foo::<u16>(x_u32);
    //~^ ERROR mismatched types
    foo::<u16>(x_u16);
    foo::<u16>(x_u8);
    //~^ ERROR mismatched types
    foo::<u16>(x_isize);
    //~^ ERROR mismatched types
    foo::<u16>(x_i64);
    //~^ ERROR mismatched types
    foo::<u16>(x_i32);
    //~^ ERROR mismatched types
    foo::<u16>(x_i16);
    //~^ ERROR mismatched types
    foo::<u16>(x_i8);
    //~^ ERROR mismatched types
    // foo::<u16>(x_f64);
    // foo::<u16>(x_f32);

    foo::<i16>(x_usize);
    //~^ ERROR mismatched types
    foo::<i16>(x_u64);
    //~^ ERROR mismatched types
    foo::<i16>(x_u32);
    //~^ ERROR mismatched types
    foo::<i16>(x_u16);
    //~^ ERROR mismatched types
    foo::<i16>(x_u8);
    //~^ ERROR mismatched types
    foo::<i16>(x_isize);
    //~^ ERROR mismatched types
    foo::<i16>(x_i64);
    //~^ ERROR mismatched types
    foo::<i16>(x_i32);
    //~^ ERROR mismatched types
    foo::<i16>(x_i16);
    foo::<i16>(x_i8);
    //~^ ERROR mismatched types
    // foo::<i16>(x_f64);
    // foo::<i16>(x_f32);

    foo::<u8>(x_usize);
    //~^ ERROR mismatched types
    foo::<u8>(x_u64);
    //~^ ERROR mismatched types
    foo::<u8>(x_u32);
    //~^ ERROR mismatched types
    foo::<u8>(x_u16);
    //~^ ERROR mismatched types
    foo::<u8>(x_u8);
    foo::<u8>(x_isize);
    //~^ ERROR mismatched types
    foo::<u8>(x_i64);
    //~^ ERROR mismatched types
    foo::<u8>(x_i32);
    //~^ ERROR mismatched types
    foo::<u8>(x_i16);
    //~^ ERROR mismatched types
    foo::<u8>(x_i8);
    //~^ ERROR mismatched types
    // foo::<u8>(x_f64);
    // foo::<u8>(x_f32);

    foo::<i8>(x_usize);
    //~^ ERROR mismatched types
    foo::<i8>(x_u64);
    //~^ ERROR mismatched types
    foo::<i8>(x_u32);
    //~^ ERROR mismatched types
    foo::<i8>(x_u16);
    //~^ ERROR mismatched types
    foo::<i8>(x_u8);
    //~^ ERROR mismatched types
    foo::<i8>(x_isize);
    //~^ ERROR mismatched types
    foo::<i8>(x_i64);
    //~^ ERROR mismatched types
    foo::<i8>(x_i32);
    //~^ ERROR mismatched types
    foo::<i8>(x_i16);
    //~^ ERROR mismatched types
    foo::<i8>(x_i8);
    // foo::<i8>(x_f64);
    // foo::<i8>(x_f32);

    foo::<f64>(x_usize);
    //~^ ERROR mismatched types
    foo::<f64>(x_u64);
    //~^ ERROR mismatched types
    foo::<f64>(x_u32);
    //~^ ERROR mismatched types
    foo::<f64>(x_u16);
    //~^ ERROR mismatched types
    foo::<f64>(x_u8);
    //~^ ERROR mismatched types
    foo::<f64>(x_isize);
    //~^ ERROR mismatched types
    foo::<f64>(x_i64);
    //~^ ERROR mismatched types
    foo::<f64>(x_i32);
    //~^ ERROR mismatched types
    foo::<f64>(x_i16);
    //~^ ERROR mismatched types
    foo::<f64>(x_i8);
    //~^ ERROR mismatched types
    foo::<f64>(x_f64);
    foo::<f64>(x_f32);
    //~^ ERROR mismatched types

    foo::<f32>(x_usize);
    //~^ ERROR mismatched types
    foo::<f32>(x_u64);
    //~^ ERROR mismatched types
    foo::<f32>(x_u32);
    //~^ ERROR mismatched types
    foo::<f32>(x_u16);
    //~^ ERROR mismatched types
    foo::<f32>(x_u8);
    //~^ ERROR mismatched types
    foo::<f32>(x_isize);
    //~^ ERROR mismatched types
    foo::<f32>(x_i64);
    //~^ ERROR mismatched types
    foo::<f32>(x_i32);
    //~^ ERROR mismatched types
    foo::<f32>(x_i16);
    //~^ ERROR mismatched types
    foo::<f32>(x_i8);
    //~^ ERROR mismatched types
    // foo::<f32>(x_f64);
    foo::<f32>(x_f32);

    foo::<u32>(x_u8 as u16);
    //~^ ERROR mismatched types
    foo::<i32>(-x_i8);
    //~^ ERROR mismatched types
}

// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_b4xporvr64s
// Floating Point Types
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_94a8v54bufn8
// Values
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
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
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
