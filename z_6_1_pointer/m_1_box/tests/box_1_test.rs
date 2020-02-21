#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

//  cargo test list_1_test -- --nocapture
#[test]
fn list_1_test() {
    use List::Cons;
    use List::Nil;
    //let list = Cons(1, Cons(2, Cons(3, Nil));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list {:?}", list);
}
