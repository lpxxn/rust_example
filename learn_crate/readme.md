
tesseract img.png stdout -l chi_sim -c preserve_interword_spaces=1
## Pacakge 和 Crate

Crate 的类型:
一 binary
一ibrary

Crate Root:

- 是源代码文件

一 Rust 编译器从这里开始,组成你的 Crate 的根 Module
一个 Package :
- 包含 1 个 Cargo.toml,它描述了如何构建这些 Crates
- 只能包含 0-1 个 library crate
- 可以包含任意数量的 binary crate
- 侄必须至少包含一个 crate (library 或 binary

Cargo 的惯例

src/main.rs:
- binary crate 的 crate root
- crate 名与 package 名相同

srcy/lib.rs:
- package 包含一个library crate
- library crate 的 crate root
- cralte 名与 package 名相同

Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary

一个 Package 可以同时包含 src/main.rs 和 srcy/lib.rs
- 一个 binary crate, 一个library crate
- 名称与 package 名相同

一个 Package 可以有多个 binary crate:
- 文件放在 src/bin
- 每个文件是单独的 binary crate



定义 module 来控制作用域和私有性

。Module:
- 在一个 crate 内,将代码进行分组
一 增加可读性,易于复用
一 控制项目 (ilem) 的私有性。public、private

植 建立 module:
- mod 关键字
- 可嵌套
- 可包含其它项 (struct、enum、 常量、trai、 函数等〉 的定义


## 路径 (Path )

*为了在 Rust 的模块中找到某个条目,需要使用路径。

。路径的两种形式:
- 绝对路径: 从 crate roof 开始,使用 crate 名 或 字面值 crate
一 相对路径, 从当前模块开始,使用 self,super 或当前模块的标识符

路径至少由一个标识符组成,标识符之间使用 ::


私有边界 (privacy boundary )

. 模块不仅可以组织代码,还可以定义私有边界。

. 如果想把 函数 或 struct 等设为私有,可以将它放到某个模块中。

. Rust 中所有的条目 (函数,方法,struct,enum,模块,常量) 默认是私有的。
. 父级模块无法访问子模块中的私有条目

. 子模块里可以使用所有祖先模块中的条目

super 关键字
. super: 用来访问父级模块路径中的内容,类似文件系统中的

pub struct

Pub 放在 struct+前:
- struct 是公共的
- struct 的字段默认是私有的

struct 的字段需要单独设置 pub 来变成共有

#### pub enum
pub 放在 enum 前:
- enum 是公共的
- erfum 的变体也都是公共的