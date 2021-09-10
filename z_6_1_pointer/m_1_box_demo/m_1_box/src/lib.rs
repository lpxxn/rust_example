// Box<T> 允许值在堆上创建，留在栈上的是指向堆数据的指针，没有性能损失。
// 适用场景
// 1. 当有一个在编译时未知大小的类型(eg string vector)，而又需要再确切大小的上下文中使用这个类型值的时候；
// 2. 当有大量数据并希望在确保数据不拷贝的情况下转移所有权；
// 3. 当希望拥有一个值并只关心它的类型是否实现了特定的trait而不是具体类型时。
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // 这样
        // crate::my_box::new_box();
        // 这样
        // use super::my_box;
        // my_box::new_box();
        // 这样
        // super::my_box::new_box();
        // assert_eq!(2 + 2, 4);
        // 这样
        use super::*;
        my_box::new_box();
    }
}
pub mod my_box {
    pub fn new_box() {
        let b = Box::new(2);
        println!("b {}", b);
    }
}
