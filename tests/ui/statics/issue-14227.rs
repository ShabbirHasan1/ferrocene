// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck

extern "C" {
    pub static symbol: u32;
}
static CRASH: u32 = symbol;
//~^ ERROR use of extern static is unsafe and requires

fn main() {}

// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
