// Issue: 100461, Try to give a helpful diagnostic even when the next struct field has an attribute.
// run-rustfix

struct Feelings {
    owo: bool
    //~^ ERROR expected `,`, or `}`, found `#`
    #[allow(unused)]
    uwu: bool,
}

impl Feelings {
    #[allow(unused)]
    fn hmm(&self) -> bool {
        self.owo
    }
}

fn main() { }

// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
