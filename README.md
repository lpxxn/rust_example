# rust_example

https://kaisery.gitbooks.io/trpl-zh-cn/content/ch01-00-getting-started.html

https://rustwiki.org/zh-CN/rust-by-example/scope/borrow/ref.html


#### Rust开发crates.io换国内中科大
```
tee $HOME/.cargo/config <<-'EOF'
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
EOF

```