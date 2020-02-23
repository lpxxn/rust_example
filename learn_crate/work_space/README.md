工作空间在顶级目录有一个 target 目录；adder 并没有自己的 target 目录。即使进入 adder 目录运行 cargo build，构建结果也位于 add/target 而不是 add/adder/target。
工作空间中的 crate 之间相互依赖。如果每个 crate 有其自己的 target 目录，为了在自己的 target 目录中生成构建结果，工作空间中的每一个 crate 都不得不相互重新编译其他 crate。通过共享一个 target 目录，
**工作空间可以避免其他 crate 多余的重复构建。**