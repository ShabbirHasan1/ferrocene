// run-rustfix

pub enum struct Range {
    //~^ ERROR `enum` and `struct` are mutually exclusive
    Valid {
        begin: u32,
        len: u32,
    },
    Out,
}

fn main() {
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
