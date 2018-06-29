use std::env; fn main() {
    match env::args().nth(1) {
        Some(target) => println!("Hello, {}!", target),
        None => println!("Hello, world!")
    }
}

