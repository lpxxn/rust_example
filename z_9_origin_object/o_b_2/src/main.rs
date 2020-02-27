fn main() {
    let btn = Button{};
    let list = List{};
    let s1 = Screen1{component: btn};
    // 不能换，
    //s1.set_component(List{});
    //                 ^^^^^^ expected struct `Button`, found struct `List`
    // 但可以重新new
    let s1 = Screen1{component: list}; 
    s1.run();

    let btn = Button{};
    let list = List{};
    let mut s2 = Screen2 { components: vec![] };
    s2.components.push(btn);

    // 下面的会有错误
    // s2.components.push(list);
    //                    ^^^ value used here after move
    s2.run();
    println!("Hello, world!");
}
// trait bound 编译器进行的方式是单态化处理，进行的是静态分发，
// 也就是说编译器在编译时就知道调用了什么方法，编译时就知道了具体类型
// 第一次传入的是谁，以后都要是谁
trait Draw {
    fn draw(&self);
}

struct Button;
impl Draw for Button {
    fn draw(&self) {
        println!("button");
    }
}

struct List;
impl Draw for List {
    fn draw(&self) {
        println!("List");
    }
}

struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

impl<T: Draw> Screen2<T> {
    pub fn run(&self) {
        for i in self.components.iter() {
            i.draw();
        }
    }
}

struct Screen1<T: Draw> {
    pub component: T,
}

impl<T: Draw> Screen1<T> {
    pub fn run(&self) {
        self.component.draw(); 
    }
    fn set_component(&mut self, c: T) {
        self.component = c;
    }
}

