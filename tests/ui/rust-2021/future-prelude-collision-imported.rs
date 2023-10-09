// run-rustfix
// edition:2018
// check-pass
#![warn(rust_2021_prelude_collisions)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod m {
    pub trait TryIntoU32 {
        fn try_into(self) -> Result<u32, ()>;
    }

    impl TryIntoU32 for u8 {
        fn try_into(self) -> Result<u32, ()> {
            Ok(self as u32)
        }
    }

    pub trait AnotherTrick {}
}

mod a {
    use crate::m::TryIntoU32;

    fn main() {
        // In this case, we can just use `TryIntoU32`
        let _: u32 = 3u8.try_into().unwrap();
        //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
        //~^^ WARNING this is accepted in the current edition
    }
}

mod b {
    use crate::m::AnotherTrick as TryIntoU32;
    use crate::m::TryIntoU32 as _;

    fn main() {
        // In this case, a `TryIntoU32::try_into` rewrite will not work, and we need to use
        // the path `crate::m::TryIntoU32` (with which it was imported).
        let _: u32 = 3u8.try_into().unwrap();
        //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
        //~^^ WARNING this is accepted in the current edition
    }
}

mod c {
    use super::m::TryIntoU32 as _;
    use crate::m::AnotherTrick as TryIntoU32;

    fn main() {
        // In this case, a `TryIntoU32::try_into` rewrite will not work, and we need to use
        // the path `super::m::TryIntoU32` (with which it was imported).
        let _: u32 = 3u8.try_into().unwrap();
        //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
        //~^^ WARNING this is accepted in the current edition
    }
}

mod d {
    use super::m::*;

    fn main() {
        // See https://github.com/rust-lang/rust/issues/88471
        let _: u32 = 3u8.try_into().unwrap();
        //~^ WARNING trait method `try_into` will become ambiguous in Rust 2021
        //~^^ WARNING this is accepted in the current edition
    }
}

fn main() {}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
