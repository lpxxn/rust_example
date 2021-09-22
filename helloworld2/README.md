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
后，默认会创建 `vendor` 文件夹，会输出
```
To use vendored sources, add this to your .cargo/config.toml for this project:

[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

```
然后创建`.cargo/config.toml`
这样在 `cargo build `时就会再重新下载，
在`cargo clean`后也不用再担心重新下载的问题了

vi create direcotry
Another way with a vanilla Vim (without extra conf or plugins). in Vim:

:!mkdir -p /folder/you/want/
:w  #save file
or

$ vim /folder/you/want/myfile.conf
$ ctrl+z # switch to the terminal then create the folders
$ mkdir -p /folder/you/want/
$ fg # return to Vim
$ :w  #save file