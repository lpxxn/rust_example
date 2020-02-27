macro_rules! hello {
    // match a pattern, output tokens
    () => { println!("Hello");};
}

fn main() {
    hello!();
    hello! [];
    hello!{};
}

//rustc --out-dir ./target ./macros_1.rs