fn main() {
    let str1 = String::from("Hello, world!");
    let s1 = &str1[0..5];
    println!("s1 = {}", s1);
    let s2 = &str1[0..=5];
    println!("s2 = {}", s2);
    let s3 = &str1[..=5];
    println!("s3 = {}", s3);
    let s3 = &str1[..];
    println!("s3 = {}", s3);

    // 字面值是slice 也就是(&str) 并且不可变
    let s4 = "abc";

    let a = [1, 2, 3, 4];
    let as1 = &a[0..3];
    println!("as1 = {:?}", as1);
}
