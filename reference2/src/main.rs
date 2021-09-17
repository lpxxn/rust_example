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
}
/*
*y 时，Rust 事实上在底层运行了如下代码：
*(y.deref())
Rust 将 * 运算符替换为先调用 deref 方法再进行普通解引用的操作，如此我们便不用担心是否还需手动调用 deref 方法了。Rust 的这个特性可以让我们写出行为一致的代码，无论是面对的是常规引用还是实现了 Deref 的类型。
deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。
 */

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

