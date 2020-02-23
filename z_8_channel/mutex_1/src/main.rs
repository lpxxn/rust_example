use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }// 离开作用域时，自动释放
    println!("number {:?}", m);
    println!("Hello, world!");
}
// Mutex<T>是一个智能指针， lock调用返回一个叫做MuteGuard的智能指针
// 内部提供了drop方法，实现当MutexGuard离开作用域时自动释放锁。