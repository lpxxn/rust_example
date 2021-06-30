## 查找库， 能得到版本信息

cargo search rand

cargo update update Update dependencies listed in Cargo.lock 只会在 Cargo.lock 里更新依赖信息。 只有在 build的时候才会具体执行

竟然有 `vendor` 命令
`cargo --help`里没有 `vendor`的说明，在`cargo --list`里
```
cargo vendor --help 
cargo help vendor
```
运行 
```
cargo vendor 
```
后，默认会创建 `vendor` 文件夹，在`Cargo.toml`里加入
```
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```
这样在 `cargo build `时就会再重新下载，
在`cargo clean`后也不用再担心重新下载的问题了