#![feature(impl_trait_in_assoc_type)]
// build-pass (FIXME(62277): could be check-pass?)

trait Bar {}
struct Dummy;
impl Bar for Dummy {}

trait Foo {
    type Assoc: Bar;
    fn foo() -> Self::Assoc;
}

impl Foo for i32 {
    type Assoc = impl Bar;
    fn foo() -> Self::Assoc {
        Dummy
    }
}

fn main() {}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
