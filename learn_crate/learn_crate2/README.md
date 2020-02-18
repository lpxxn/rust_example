```
cargo new learn_crate2
cd learn_crate2
cargo new --lib mylib
```
Cargo.toml
```
mylib = {path = "./mylib"}
```