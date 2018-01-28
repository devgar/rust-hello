use std::env; fn main() {
    if let Some(arg1) = env::args().nth(1) {
        println!("Hello, {}!", arg1);
    } else {
        println!("Hello, world!");
    }
}

