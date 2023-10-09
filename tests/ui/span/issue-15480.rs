// run-rustfix
fn id<T>(x: T) -> T { x }

fn main() {
    let v = vec![
        &id(3)
    ];
    //~^^ ERROR temporary value dropped while borrowed

    for &&x in &v {
        println!("{}", x + 3);
    }
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
