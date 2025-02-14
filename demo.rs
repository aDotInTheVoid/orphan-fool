use std::fmt;

impl fmt::Display for Vec<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn main() {
    let x = vec![1, 2, 3];

    println!("x = {x}");
}
