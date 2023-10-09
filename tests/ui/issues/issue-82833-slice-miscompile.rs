// run-pass
// compile-flags: -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Copt-level=0 -Cdebuginfo=2

// Make sure LLVM does not miscompile this.

fn make_string(ch: char) -> String {
    let mut bytes = [0u8; 4];
    ch.encode_utf8(&mut bytes).into()
}

fn main() {
    let ch = '😃';
    dbg!(ch);
    let string = make_string(ch);
    dbg!(string);
}

// ferrocene-annotations: um_rustc_C_inline_threshold
// ferrocene-annotations: um_rustc_C_debuginfo
// ferrocene-annotations: um_rustc_C_link_dead_code
