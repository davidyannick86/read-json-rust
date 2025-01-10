use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}

fn main() {
    println!("Hello, world!");
}
