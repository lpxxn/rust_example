# rust_example

https://kaisery.gitbooks.io/trpl-zh-cn/content/ch01-00-getting-started.html

https://rustwiki.org/zh-CN/rust-by-example/scope/borrow/ref.html

https://wiki.jikexueyuan.com/project/rust-primer/

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
或者这个

```
https://rsproxy.cn/
```

## doc

```
rustup doc
rustup docs --book
```

### update
```
rustup -V
rustc -V

# 更新工具链
rustup update
# 更新 rustup
rustup self update
rustup self upgrade-data
rustup -V
rustc -V
```
###

```
rustup toolchain install nightly
```

Now Rust nightly is installed, but not activated. To test it out you can run a command from the nightly toolchain like

```
rustup run nightly rustc --version
```
rustc 1.9.0-nightly (02310fd31 2016-03-19)
But more likely you want to use it for a while. To switch to nightly globally, change the default with rustup default nightly:

```
rustup default nightly

rustup default stable
```
info: using existing install for 'nightly'
info: default toolchain set to 'nightly'

  nightly unchanged: rustc 1.9.0-nightly (02310fd31 2016-03-19)
