
https://rustwiki.org/zh-CN/rust-by-example/crates/link.html


让我们创建一个库，然后看看如何把它链接到另一个 crate。
```
pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

```

```
$ rustc --crate-type=lib rary.rs
$ ls lib*
library.rlib

```
默认情况下，库会使用 crate 文件的名字，前面加上 “lib” 前缀，但这个默认名称可以 使用 crate_name 属性 覆盖。


### extern crate
要把上一节创建的库链接到一个 crate，必须使用 extern crate 声明。这不仅会 链接库，还会用一个与库名相同的模块来存放库里面的所有项。于模块的可见性规则也 适用于库。
```
// 链接到 `rary` 库，导入其中的项
extern crate rary;

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}

```
```
# library.rlib 是已编译好的库的路径，这里假设它在同一目录下：
$ rustc executable.rs --extern rary=library.rlib && ./executable
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`

```

crate
crate_type 属性可以告知编译器 crate 是一个二进制的可执行文件还是一个 库（甚至是哪种类型的库），crate_name 属性可以设定 crate 的名称。

不过，一定要注意在使用 cargo 时，这两种类型时都没有作用。由于大多数 Rust 工程都使用 cargo，这意味着 crate_type 和 crate_name 的作用事实上很有限。
```
// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

```
当用到 crate_type 属性时，就不再需要给 rustc 命令加上 --crate-type 标记。
```
$ rustc lib.rs
$ ls lib*
library.rlib

```

