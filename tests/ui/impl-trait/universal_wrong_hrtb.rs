trait Trait<'a> {
    type Assoc;
}

fn test_argument_position(x: impl for<'a> Trait<'a, Assoc = impl Copy + 'a>) {}
//~^ ERROR `impl Trait` can only mention lifetimes from an fn or impl

fn main() {}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
