fn main() {
    println!("Hello, {}!", match std::env::args().nth(1) {
        Some(target) => target, None => String::from("world")
    });
}

