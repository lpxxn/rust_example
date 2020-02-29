fn main() {
    let x = vec![1, 2, 3];
    println!("x {:?}", x);
    let equal_to_x = move |z| z == x;
    // 上面已经 move了，所以下面这个会报错
    //println!("x {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let e = E {
        a: "fn_once".to_string(),
    };
    // 这样加个move，看看程序执行输出顺序有什么不同
    //let f = move || println!("fn once calls: {:?}", e);
    let f = || println!("fn once closure calls: {:?}", e);
    fn_once(f);
    println!("e {:?}", e);

    let mut e = E {
        a: "fn_once".to_string(),
    };
    let f = || {
        println!("FnMut closure calls: {:?}", e);
        e.a = "fn_mut".to_string();
    };
    fn_mut(f);

    println!("e {:?}", e);

    let e = E {
        a: "fn_once".to_string(),
    };
    let f = || {
        println!("Fn closure calls: {:?}", e);
    };
    fn_immut(f);
    println!("e {:?}", e);

    println!("main ended");
    // https://www.dazhuanlan.com/2019/12/09/5dee50f786c92/
}

/* 闭包可以通过三种方式捕获其环境，他们对应函数的三种获取参数的方式: 获取所有权、可恋借用、不可变借用。
这三种捕获值的方式被编码为如下三个Fn trait:
1. FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境。为了消费捕获到的变量，闭包必须
    获取其所有权并在定义闭包时将其移进闭包。其名称Once部分代表了闭包不能多次获取相同变量的所有权。
2. FnMut 获取可变的借用值，所以可以改变其环境。
3. Fn 从其环境获取不可变的借用值。
当创建一个闭包时，rust会根据其环境中的变量来推断我们希望如何引用环境。由于所有闭包老阳可以被调用至少
一次，因此所有的闭包都实现了FnOnce。没有移动被捕获变量的所有权到闭包的闭包也实现了Fnmut,而不需要参考捕获
变量进行可变访问的闭包实现了Fn
https://www.dazhuanlan.com/2019/12/09/5dee50f786c92/
*/

#[derive(Debug)]
struct E {
    a: String,
}

impl Drop for E {
    fn drop(&mut self) {
        println!("destroyed struct E");
    }
}
// FnOnce 这种类型的闭包会获取变量的所有权，生命周期只能是当前作用域，之后就会被释放了。
fn fn_once<F>(func: F)
where
    F: FnOnce(),
{
    println!("fn_once begins");
    func();
    // 但是如果闭包运行两次
    // func();// value used here after move
    /*
    还是回到 FnOnce 的定义，
    参数类型是 self，所以在 func 第一次执行完之后，之前捕获的变量已经被释放了，
    所以已经无法在执行第二次了。如果想要运行多次，可以使用 FnMut/FnOnce 。
    #[lang = "fn_once"]
    pub trait FnOnce<Args> {
        type Output;
        extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
    }
    */
    println!("fn_once ended");
}

/*
FnMut
#[lang = "fn_mut"]
pub trait FnMut<Args>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
参数类型是 &mut self，所以，这种类型的闭包是可变借用，会改变变量，但不会释放该变量。所以可以运行多次。
可以看出 FnMut 类型的闭包是可以运行多次的，且可以修改捕获变量的值。
*/

fn fn_mut<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

/*
    Fn
#[lang = "fn"]
pub trait Fn<Args>: FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}
参数类型是 &self，所以，这种类型的闭包是不可变借用，不会改变变量，也不会释放该变量。所以可以运行多次。

例子:
*/

fn fn_immut<F>(mut func: F)
where
    F: Fn(),
{
    func();
    func();
}
