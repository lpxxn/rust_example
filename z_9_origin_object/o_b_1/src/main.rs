use aver_collect::*;
fn main() {
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

    println!("Hello, world!");
}
