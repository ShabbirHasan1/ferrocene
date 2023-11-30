// revisions: rust2015 rust2018
//[rust2018] edition:2018

trait WithType<T> {}
trait WithRegion<'a> { }

trait Foo { }

impl<T> Foo for Vec<T>
where
    T: WithType<&u32>
//[rust2015,rust2018]~^ ERROR `&` without an explicit lifetime name cannot be used here
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
