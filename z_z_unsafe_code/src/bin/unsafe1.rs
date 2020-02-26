// 解引用裸指针
// 不可变和可变的，分别写作 *const T, *mut T
// 允许忽略借用规则，可以同时拥有不可变和可变的指针，或者是多个指向相同位置的可变指针
// 不保证指向的内存是有效的
// 允许为空
// 不能实现任何自动清理的功能

fn main() {
    let mut num = 1;
    // 创建不可变和可变的裸指针，可以在安全的代码中，
    // 只是不能在解引用裸指针，只能在unsafe里
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        print!("r1: {}", *r1);
        print!("r2: {}", *r2);
    }
    // let a = 0x1234usize;
    // let r = add as *const i32;
    println!("{}", a);
}