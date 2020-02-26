cargo new helloworld

cargo run

cargo build

cargo check [检查代码]

[cargo fmt 格式化代码](https://github.com/rust-lang/rustfmt)

### rustc
rustc --out-dir ./target strategy_pattern.rs 
 
### release
cargo build --release
./target/release/helloworld 
cargo run --release


### doc
cargo doc
cargo doc --open

cargo test // 在z_6_1_pointer my_1_box里有 test

### corss

https://github.com/rust-embedded/cross
$ cargo install cross

cross build --target x86_64-unknown-linux-gnu
cross build --target x86_64-unknown-linux-musl
###

https://rustwiki.org/zh-CN/rust-by-example/index.html
