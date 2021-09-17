use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    let s = "hi".to_string();  // : String
    let b = &*s;   // equivalent to `&(*s)`
    let c = &(*s);

    let d = std::ops::Deref::deref(&s);
    let x = 5;

    let y = &x;
    let y1 = *y;
    let z = *std::ops::Deref::deref(&y);
    /*
    So, *s is actually *std::ops::Deref::deref(&s), in which the deref() function has the return type &str which is then dereferenced again. Thus, *s has the type str (note the lack of &).
    In general, &* means to first dereference (*) and then reference (&) a value. In many cases, this would be silly, as we'd end up at the same thing.
     */
    let a = A;
    let b = a.get_b();

    let a = Box::new(A);
    let b = a.get_b();
    std::mem::drop(a);
    // a和b的生命周期一样长，如果 drop掉，也就不能用b了
    //let c = b;

    let s = Box::new(String::from("abcd"));
    hello(&*s);
    hello(&s);
    let s2 :&Box<String> = &s;
    /*
    注意这个【引用】
    Deref 强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。
    其将实现了 Deref 的类型的[*引用*]转换为原始类型通过 Deref 所能够转换的类型的引用。当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，Deref 强制转换将自动发生。这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。
    Deref coercion is a convenience that Rust performs on arguments to functions and methods.
    Deref coercion works only on types that implement the Deref trait. Deref coercion converts such a type into a reference to another type.
    For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str. Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
     */

}
/*
*y 时，Rust 事实上在底层运行了如下代码：
*(y.deref())
Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作，如此我们便不用担心是否还需手动调用 deref 方法了。Rust 的这个特性可以让我们写出行为一致的代码，无论是面对的是常规引用还是实现了 Deref 的类型。
deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。
注意，每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换
类似于如何使用 Deref trait 重载不可变引用的 * 运算符，Rust 提供了 DerefMut trait 用于重载可变引用的 * 运算符。

Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：

当 T: Deref<Target=U> 时从 &T 到 &U。
当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
当 T: Deref<Target=U> 时从 &mut T 到 &U。

 */

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct A;
struct B;
impl Deref for A {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        let x = &B;
        return x
    }
}

fn foo(a: A) -> Option<()> {
    let v = &&a;
    bar(v);
    bar(&&a);
    bar((&a).deref());
    bar(&*(&a));

    let a1 = &a;
    bar(a1.deref());
    Some(())
}

fn bar(b: &B) {

}

impl A {
    fn get_b(&self)  -> &B {
        &B
    }
}

// fn get_b() -> &B {
//     &B
// }

