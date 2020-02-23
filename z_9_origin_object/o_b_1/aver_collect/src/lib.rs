pub trait Draw: std::fmt::Debug {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,// trait 对象，使用dyn关键字
}

impl Screen {
    pub fn run(&self) {
        for i in self.components.iter() {
            i.draw();
        }
    }
}

// trait 单态化处理，是静态分法，编译时就知道了具体类型
// 第一次传入的是谁，以后都要是谁
// #[derive(Debug)]
// struct Screen2<T: Draw> {
//     pub components: Vec<T>
// }

// impl<T: Draw> Screen2<T> {
//     pub fn run(&self) {
//         for i in self.components.iter() {
//             i.draw();
//         }
//     }
// }

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub text: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("draw button, w: {}, h: {}, label: {}", self.width, self.height, self.text);
    }
}

#[derive(Debug)]
pub struct SelectBox{
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw selectBox w: {}, h: {}, options: {:?}", self.width, self.height, self.options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn screen() {
        let btn = Button{
            width: 100,
            height: 35,
            text: "abcdef".to_string(),
        };
        println!("button {:?}", btn);

        let selb = SelectBox {
            width: 100,
            height: 35, 
            options: vec!["a".to_string(), "b".to_string()],
        };
        let mut screen = Screen{
            components: vec![],
        };
        screen.components.push(Box::new(btn));
        screen.components.push(Box::new(selb));
        screen.run();

        // let s2 = Screen2{
        //     components: vec![],
        // };
        // s2.components.push(selb);
        // s2.components.push(btn);
        // s2.run();
    }
}
