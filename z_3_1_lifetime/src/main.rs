//1. rust 的每一个引用都有他的生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，
// 就和大部分类型是可以推断的一样
//2. 生命周期的主要目标是避免悬垂引用。
//3. Rust编译器使用借用检查器来检查生命周期是否有效。

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("abc");
    let s2 = String::from("cdef");
    {
     let s3 = longest(s1.as_str(), s2.as_str());
    }

    let n = String::from("hello");
    let a = A{name: n.as_str()};
    a.do_something();
    println!("a = {:?}", a);
    println!("a name = {}", a.get_name());
    println!("a get_str2 = {}", a.get_str2(s2.as_str()));
}

fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// 结构体生命周期
#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

impl<'b> A<'b> {
    fn do_something(&self) -> i32 {
        3
    }
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_str(&self, s: &str) -> &str {
    // fn get_str(&'b self, s: &str) -> &'b str
        self.name
    }
    fn get_str2<'c>(&self, s: &'c str) -> &'c str {
        s
    }
}

// 生命周期省略
fn get_a_str(s: &str) -> &str {
    s
}
/*
编译器有三条规则判断引用何时不需要生命周期
生命周期注解省略规则适用于fn定义以及impl块定义，如下：
 1. 每个引用的参数都有他自己在的生命周期参数，例：
        一个引用参数的函数，其中有一个生命周期: fn foo<'a>(x: &'a i32);
        两个引用参数的函数，则有两个生命周期：fn foo<'a 'b>(x: &'a i32, y: &'b i32)
        以此类推
 2. 如果只有一个输入生命周期参数，那么他被赋予所有输出生命周期参数：
    fn foo(x: &i32) -> &i32 == fn foo<'a>(x: &'a i32) -> &'a i32
 
 3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为&self 或者 &mut self 那么self的生命周期被赋予所有的转出生命周期
 参数。例：
    fn eg(&self, x: &str, y: &str) -> &str
*/
/*
fn main() {
    let r;
    {
        let x = 1;
        r = &x;
    }
}
// MIR
https://play.rust-lang.org/?version=stable&mode=debug&edition=2018

// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn  main() -> () {
    let mut _0: ();                      // return place in scope 0 at src/main.rs:3:11: 3:11
    let _1: &i32;                        // in scope 0 at src/main.rs:4:9: 4:10
    let mut _3: &i32;                    // in scope 0 at src/main.rs:7:13: 7:15
    scope 1 {
        debug r => _1;                   // in scope 1 at src/main.rs:4:9: 4:10
        let _2: i32;                     // in scope 1 at src/main.rs:6:13: 6:14
        scope 2 {
            debug x => _2;               // in scope 2 at src/main.rs:6:13: 6:14
        }
    }

    bb0: {
        StorageLive(_1);                 // bb0[0]: scope 0 at src/main.rs:4:9: 4:10
        StorageLive(_2);                 // bb0[1]: scope 1 at src/main.rs:6:13: 6:14
        _2 = const 1i32;                 // bb0[2]: scope 1 at src/main.rs:6:17: 6:18
                                         // ty::Const
                                         // + ty: i32
                                         // + val: Value(Scalar(0x00000001))
                                         // mir::Constant
                                         // + span: src/main.rs:6:17: 6:18
                                         // + literal: Const { ty: i32, val: Value(Scalar(0x00000001)) }
        StorageLive(_3);                 // bb0[3]: scope 2 at src/main.rs:7:13: 7:15
        _3 = &_2;                        // bb0[4]: scope 2 at src/main.rs:7:13: 7:15
        _1 = move _3;                    // bb0[5]: scope 2 at src/main.rs:7:9: 7:15
        StorageDead(_3);                 // bb0[6]: scope 2 at src/main.rs:7:14: 7:15
        StorageDead(_2);                 // bb0[7]: scope 1 at src/main.rs:8:5: 8:6
        StorageDead(_1);                 // bb0[8]: scope 0 at src/main.rs:9:1: 9:2
        return;                          // bb0[9]: scope 0 at src/main.rs:9:2: 9:2
    }
}

也可以试一下这个
fn main() {
    let r;
    let x = 1;
    r = &x;
    let c = *r;
    let d = r;
}


*/