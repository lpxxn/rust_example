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

#[derive(Debug)]
struct Checkbox;
impl Draw for Checkbox {
    fn draw(&self) -> String {
       format!("checkbox: {:?}", self)
    }
}

fn draw1(x: Box<dyn Draw>) {
    println!("{}",x.draw());
}

fn draw2(x: &dyn Draw) {
    println!("{}",x.draw());
}
//cargo test --bin r7trait d1::trait2_dyn::test_draw -- --show-output
#[test]
fn test_draw() {
    let button = Button;
    let checkbox = Checkbox;
    draw1(Box::new(button));
    draw1(Box::new(checkbox));
}