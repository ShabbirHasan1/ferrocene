// edition:2018
// run-rustfix

trait WithType<T> {}
trait WithRegion<'a> { }

#[allow(dead_code)]
struct Foo<T> {
    t: T
}

impl<T> Foo<T>
where
    T: WithType<&u32>
//~^ ERROR `&` without an explicit lifetime name cannot be used here
{ }

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
