use std::path::{Path, PathBuf};


fn main() {
    let _tis_an_instants_play: String = "'Tis a fond Ambush—"; //~ ERROR mismatched types
    let _just_to_make_bliss: PathBuf = Path::new("/ern/her/own/surprise");
    //~^ ERROR mismatched types

    let _but_should_the_play: String = 2; // Perhaps surprisingly, we suggest .to_string() here
    //~^ ERROR mismatched types

    let _prove_piercing_earnest: Vec<usize> = &[1, 2, 3]; //~ ERROR mismatched types
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
