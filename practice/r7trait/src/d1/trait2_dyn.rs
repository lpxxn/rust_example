use std::fmt::format;

trait Draw {
    fn draw(&self) -> String;
}

#[derive(Debug)]
struct Button;
impl Draw for Button {
    fn draw(&self) -> String {
        format!("button: {:?}", *self)
    }
}
// impl Draw for &Button {
//     fn draw(&self) -> String {
//         format!("button: {:?}", *self)
//     }
// }
//

#[derive(Debug)]
struct Checkbox;
impl Draw for Checkbox {
    fn draw(&self) -> String {
       format!("checkbox: {:?}", self)
    }
}
// impl Draw for &Checkbox {
//     fn draw(&self) -> String {
//         format!("checkbox: {:?}", self)
//     }
// }

fn draw1(x: Box<&dyn Draw>) {
    println!("{}",x.draw());
}

fn draw2(x: &dyn Draw) {
    println!("{}",x.draw());
}
/*
draw1 参数是Box<dyn Draw> 开工的特征对象，该对象是通过Box::new(x)创建
通过 Box::new 包装成 Box<dyn Draw> 特征对象
Box<dyn Draw>，任何实现了 Draw 特征的类型，都可以存放其中。
draw2 参数是&dyn Draw 形式的特征对象，该特征对象是通过 &x的方式创建的
dyn 关键字只用在特征对象的类型声明上，在创建时无需傅dyn
https://doc.rust-lang.org/std/keyword.dyn.html
 */
//cargo test --bin r7trait d1::trait2_dyn::test_draw -- --show-output
#[test]
fn test_draw() {
    let button = Button;
    let checkbox = Checkbox;
    draw1(Box::new(&button));
    draw1(Box::new(&checkbox));


    let screen = Screen{
        components: vec![&button, &checkbox],
    };
    screen.run();
    draw2(&button);
    draw2(&checkbox);
}

// 不参返回 dyn
// fn get_ele(t: i32) -> (dyn Draw)

struct Screen<'a> {
    components: Vec<&'a dyn Draw>,
}

impl Screen<'_> {
    fn run(&self) {
        for component in self.components.iter() {
            println!("{}",component.draw());
        }
    }
}