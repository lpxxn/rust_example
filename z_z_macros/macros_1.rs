macro_rules! hello {
    // match a pattern, output tokens
    () => { println!("Hello");};
}

fn main() {
    hello!();
    hello! [];
    hello!{};
}

/*
#[derive(Serialize)]
struct Fool {
    bar: usize,
}
generqte someting like..
impl Serialize fro Fool {
    ...
}
*/
// https://github.com/dtolnay/cargo-expand

//rustc --out-dir ./target ./macros_1.rs