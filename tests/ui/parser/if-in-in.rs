// run-rustfix

fn main() {
    for i in in 1..2 { //~ ERROR expected iterable, found keyword `in`
        println!("{}", i);
    }
}

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
