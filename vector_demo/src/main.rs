fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("v = {:?}", v);

    let v2 = vec![1, 2, 3];
    let one: i32 = v[0];
    println!("one: {}", one);
    let one2: &i32 = &v[0];
    println!("one: {}", *one2);
    v.push(5);
    match v.get(2) {
        Some(x) => println!("x = {}", x),
        None => println!("none"),
    }
    match v.get(20) {
        Some(x) => println!("x = {}", x),
        None => println!("none"),
    }

    // update
    v.push(3);
    let v1 = v[0];
    // 注意 move 最后面有解释
    // for ele in v { // value moved here
    //     println!("ele = {}", ele);
    // }
    // value borrowed here after move
    // println!("v = {:?}", v);

    for ele in &v {
        println!("ele = {}", ele);
    }

    // 修改
    for ele in &mut v {
        *ele += 1
    }
    println!("=======");
    for ele in &v {
        println!("ele = {}", ele);
    }
    println!("v = {:?}", v);
    // 枚举
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }
    let c = vec![
        Context::Text(String::from("hello")),
        Context::Int(12),
        Context::Float(2.0),
    ];

    //
    let mut v = vec![1, 2, 3, 4, 5];
    let mut first = v[0];
    v.push(8);
    println!("first = {}", first);
    first += 22;
    println!("v = {:?}", v);

    let sv = vec![
        "a".to_string(),
        "bde".to_string(),
        "adljei".to_string(),
        "de".to_string(),
    ];
    let max = sv.iter().max_by_key(|s| s.len()).unwrap();
    println!("max {}", max);
    let max = sv.iter().max().unwrap();
    println!("max {}", max);
    //let _: () = max.len();
    //let _: () = f64::ceil(max.len() / 2);
    let a = max.len() as i32 % 2;
    let a = (5 as f64 / 2 as f64).ceil() as i32;
    println!("a {}", a);
    println!("{}", "*".repeat((2.5 as f32).ceil() as usize));
    // let _: () = a; i32
}

/*
rust   move   borrowing
我有下面的Rust程序。

fn main() {
    let v = vec![100, 32, 57];
    for i in v {
        println!("{}", i);
    }

    println!("{:?}", v);
}
当我运行它时，我得到：

error[E0382]: borrow of moved value: `v`
 --> src\main.rs:7:22
  |
2 |     let v = vec![100, 32, 57];
  |         - move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
3 |     for i in v {
  |              -
  |              |
  |              value moved here
  |              help: consider borrowing to avoid moving into the for loop: `&v`
...
7 |     println!("{:?}", v);
  |                      ^ value borrowed here after move
错误指出for i in v中for i in v发生了移动 。 但是我只是使用由let v = vec![100, 32, 57]定义的变量v 。 不像let v2 = v; for i in v2 ... let v2 = v; for i in v2 ... ，它将值从v移到v2 。 有人可以帮忙解释一下吗？

rust move borrowing
1 个回复
按投票数排序按时间排序
===============>>#1 票数：4
正如https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops所说，

for表达式是一种语法构造，用于循环执行由std::iter::IntoIterator实现提供的元素。

Vec 实现了IntoIterator ，使您可以通过使用它来拥有Vec实例的元素：

创建一个消耗迭代器，即一个将每个值从向量中移出（从开始到结束）的迭代器。 调用此向量后将无法使用该向量。

（如错误消息所示，解决此问题的方法是在&v而不是v上循环，借用它的元素而不是拥有它们。您可以for &i in &v循环以保持i的类型。）

从高层看，拥有v的元素似乎是不必要的，因为它们是可复制的，但是没有特殊的实现方式允许这样做。 IntoIterator.into_iter()采用self ，这意味着for循环始终消耗要迭代的值。

===============>>#2 票数：0
Vec<T>存储在堆中，这意味着它不实现Copy特征，因此for循环导致v移动（即，获得它的所有权）。 编译器建议使用v代替，这意味着for循环完成后将返回所有权。 要做到这一点，你只需把一个符号&前面v ：

fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    println!("{:?}", v);
}
如果引用和所有权的规则不清楚，我建议阅读Rust指南以更熟悉这些概念： https ： //doc.rust-lang.org/book/ch04-02-references-and-borrowing。 html
*/
