
in the r17macro folder
cargo new --lib hello_macro_derive


首先，在 `hello_macro_derive/Cargo.toml` 文件中添加以下内容：

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

其中 `syn` 和 `quote` 依赖包都是定义过程宏所必需的，同时，还需要在 `[lib]` 中将过程宏的开关开启 : `proc-macro = true`。
