// run-rustfix
struct X {
    x: String,
}

impl Drop for X {
    fn drop(&mut self) {
        println!("value: {}", self.x);
    }
}

fn unwrap(x: X) -> String {
    let X { x: y } = x; //~ ERROR cannot move out of type
    y.to_string()
}

fn main() {
    let x = X { x: "hello".to_string() };
    let y = unwrap(x);
    println!("contents: {}", y);
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
