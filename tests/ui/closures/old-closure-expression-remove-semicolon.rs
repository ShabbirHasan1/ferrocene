// run-rustfix

fn foo() -> i32 {
    0
}

fn main() {
    let _x: i32 = {
        //~^ ERROR mismatched types
        foo(); //~ HELP remove this semicolon to return this value
    };
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_8gpcpvc99pxj
// Call Conformance
