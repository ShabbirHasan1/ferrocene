fn main() {}

struct X;

impl X {
    type Y;
    //~^ ERROR associated type in `impl` without body
    //~| ERROR inherent associated types are unstable
    type Z: Ord;
    //~^ ERROR associated type in `impl` without body
    //~| ERROR bounds on `type`s in `impl`s have no effect
    //~| ERROR inherent associated types are unstable
    type W: Ord where Self: Eq;
    //~^ ERROR associated type in `impl` without body
    //~| ERROR bounds on `type`s in `impl`s have no effect
    //~| ERROR inherent associated types are unstable
    type W where Self: Eq;
    //~^ ERROR associated type in `impl` without body
    //~| ERROR inherent associated types are unstable
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
