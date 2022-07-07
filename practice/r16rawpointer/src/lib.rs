use std::slice;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

#[test]
fn it_works() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r1: *const i32 = &num;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }
}
/*
以上代码另一个值得注意的点就是：除了使用 `as` 来显式的转换，
我们还使用了隐式的转换方式 `let c: *const i32 = &a;`。
在实际使用中，我们建议使用 `as` 来转换，因为这种显式的方式更有助于提醒用户：你在使用的指针是裸指针，需要小心。
 */

// 猎取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
    let string = "hello world!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
    }
}

#[test]
fn test_raw_pointer() {
    let (pointer, length) = get_memory_location();
    let msg = get_str_at_location(pointer, length);
    println!("the {} bytes at 0x{:X} stored: {}", length, pointer, msg);
    //如果想知道为什么处理裸指针需要 unsafe， 可以试着反注释下面代码
    //let msg = get_str_at_location(10, 10);
    //println!("the {} bytes at 0x{:X} stored: {}", length, pointer, msg);
}


unsafe fn dangerous() {}
/*
还有，`unsafe` 无需俄罗斯套娃，在 `unsafe` 函数体中使用 `unsafe` 语句块是多余的行为。
 */

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//
//     assert!(mid <= len);
//
//     (&mut slice[..mid], &mut slice[mid..])
// }
/*
error[E0499]: cannot borrow `*slice` as mutable more than once at a time
 --> src/main.rs:6:30
  |
1 | fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  |                        - let's call the lifetime of this reference `'1`
...
6 |     (&mut slice[..mid], &mut slice[mid..])
  |     -------------------------^^^^^--------
  |     |     |                  |
  |     |     |                  second mutable borrow occurs here
  |     |     first mutable borrow occurs here
  |     returning this value requires that `*slice` is borrowed for `'1`
```

对于 Rust 的借用检查器来说，它无法理解我们是分别借用了同一个切片的两个不同部分，但事实上，这种行为是没任何问题的，毕竟两个借用没有任何重叠之处。总之，不太聪明的 Rust 编译器阻碍了我们用这种简单且安全的方式去实现，那只能剑走偏锋，试试 `unsafe` 了。
 */
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

#[test]
fn test_raw_ptr2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r: &mut [i32] = &mut v[..];
    r[0] = 0;
    let (a, b) = split_at_mut(r, 3);
    println!("a: {:?}, b: {:?}", a, b);
    println!("v[0]: {:?}", v.get(0));
    println!("v[1]: {:?}", v.get(1));
}