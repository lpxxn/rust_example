https://benjamincongdon.me/blog/2019/12/04/Fast-Rust-Docker-Builds-with-cargo-vendor/


```
curl --location --request POST 'http://localhost:8880/idx1' \
--header 'Content-Type: application/json' \
--data-raw '{
"username": "hi"
}'
```


curl --location --request POST 'http://test3-cafeteria.meican.com/users/login' \
--header 'Content-Type: application/json' \
--data-raw '{                                                         
"type": 2,
"username": "cashierT",
"password": "C.123456"
}'


https://github.com/dtolnay/cargo-expand
```
cargo expand > expand.rs 
```

```
rustup target add x86_64-unknown-linux-musl

cargo build --target=x86_64-unknown-linux-musl
cargo run --target=x86_64-unknown-linux-musl

cargo build --release --target=x86_64-unknown-linux-musl

Failed to find tool. Is `musl-gcc` installed
https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/


brew install filosottile/musl-cross/musl-cross
这样
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc

在 .cargo config.toml里
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"

CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl

CC_x86_64_unknown_linux_musl="x86_64-linux-musl-gcc" cargo build --release --target=x86_64-unknown-linux-musl


```