fn main() {
    println!("Hello, world!");
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle {
             width,
             height,
        }
    }
}

/*
self、&self 和 &mut self
接下里的内容非常重要，请大家仔细看。在 area 的签名中，我们使用 &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写（注意大小写）。在一个 impl 块内，Self 指代被实现方法的结构体类型，self 指代此类型的实例，换句话说，self 指代的是 Rectangle 结构体实例，这样的写法会让我们的代码简洁很多，而且非常便于理解：我们为哪个结构体实现方法，那么 self 就是指代哪个结构体的实例。

需要注意的是，self 依然有所有权的概念：

self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
&self 表示该方法对 Rectangle 的不可变借用
&mut self 表示可变借用
 */
