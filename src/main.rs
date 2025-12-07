use std::env::args;

fn main() {
    let arg = args()
        .into_iter()
        .skip(1)
        .next()
        .expect("to have a single argument");

    window::window(arg).unwrap();
}
