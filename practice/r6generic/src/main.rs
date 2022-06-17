use std::fmt::{Debug, Formatter};

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(1i32, 4));
    println!("add f64: {}", add(1.0f64, 4.1));

    let arr: [i32;3] = [1, 3, 5];
    display_array(&arr);
    display_array2(arr);
    display_array2(arr);

    let arr: [i32;2] = [1, 2];
    display_array(&arr)
}
fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a+b
}

struct Point<T,U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V,W>) -> Point<T, W> {
        return Point{x: self.x, y: other.y}
    }
}
// 为具体的泛型类型实现方法 下面是为 i32的point做的实现, 只有两个都是i32时才有这个方法
impl Point<i32, i32> {
    fn distance_from_origin(&self) -> i32 {
        self.x + self.y
    }
}
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr)
}
/*
通过引用，我们可以很轻松的解决处理任何类型数组的问题，但是如果在某些场景下引用不适宜用或者干脆不能用呢？你们知道为什么以前 Rust 的一些数组库，在使用的时候都限定长度不超过 32 吗？因为它们会为每个长度都单独实现一个函数，简直。。。毫无人性。难道没有什么办法可以解决这个问题吗？
好在，现在咱们有了 const 泛型，也就是针对值的泛型，正好可以用于处理数组长度的问题：
我们定义了一个类型为 `[T; N]` 的数组，其中 `T` 是一个基于类型的泛型参数，这个和之前讲的泛型没有区别，而重点在于 `N` 这个泛型参数，它是一个基于值的泛型参数！因为它用来替代的是数组的长度。
`N` 就是 const 泛型，定义的语法是 `const N: usize`，表示 const 泛型 `N` ，它基于的值类型是 `usize`。
在泛型参数之前，Rust 完全不适合复杂矩阵的运算，自从有了 const 泛型，一切即将改变。
 */

fn display_array2<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("display_array2 {:?}", arr)
}

struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/*
目前，const 泛型参数只能使用以下形式的实参:
一个单独的 const 泛型参数
一个字面量 (i.e. 整数, 布尔值或字符).
一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
 */

fn foo<const N: usize>() {}
fn bar<T, const M:usize>() {
    foo::<M>(); // ok，符合第一种
    foo::<123>(); // ok 符合第二种
    foo::<{18*18 + 123}>(); // ok 符合第三种，注意 { 表达式 }
    // 违背有第三种
    //foo::<{M +1}>();
    // foo::<{ std::mem::size_of::<T>()}>();

    let _: [u8; M];
}