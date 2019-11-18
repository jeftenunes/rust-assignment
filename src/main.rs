use std::env::args;

fn main() {
    args()
        .filter(|p| p.get(0..1).unwrap() == "R")
        .for_each(|f| println!("Hello, {}", f));
}
