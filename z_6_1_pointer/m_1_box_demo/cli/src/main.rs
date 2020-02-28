use std::rc::Rc;
//use m_1_box;
fn main() {
    m_1_box::my_box::new_box();
    println!("Hello, world!");

    let mut a = Rc::new(A{v: 1});
    println!("a {:?}", a);
    if let Some(v) = Rc::get_mut(&mut a){
        v.v = 3;
    } else {
        println!("get_mut is none");
    }
    println!("a {:?}", a);
    // 只要是有clone 再使用get_mut 就会返回 None
    let b = a.clone();
    if let Some(v) = Rc::get_mut(&mut a){
        v.v = 3;
    } else {
        println!("get_mut is none");
    }

}

#[derive(Debug)]
struct A {
    v: i32,
}
