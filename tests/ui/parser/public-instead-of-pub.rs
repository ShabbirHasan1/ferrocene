// Checks what happens when `public` is used instead of the correct, `pub`
// edition:2018
// run-rustfix
public struct X;
//~^ ERROR expected one of `!` or `::`, found keyword `struct`
//~^^ HELP write `pub` instead of `public` to make the item public

fn main() {}

// ferrocene-annotations: fls_wb86edg02t6a
// Items
