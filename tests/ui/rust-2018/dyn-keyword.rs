// edition:2015
// run-rustfix

#![allow(unused_variables)]
#![deny(keyword_idents)]

fn main() {
    let dyn = (); //~ ERROR dyn
    //~^ WARN this is accepted in the current edition
}

// ferrocene-annotations: fls_lish33a1naw5
// Keywords
//
// ferrocene-annotations: fls_mec5cg5aptf8
// Strict Keywords
//
// ferrocene-annotations: fls_21vnag69kbwe
// Identifiers
