#[test]
fn it_works() {
    // 通过 Rc实现Mutex的多所有权
    let counter = Rc::new(Mutex::new(0));
}
