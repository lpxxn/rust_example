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

    // slice 必需是utf-8的字符串，如果是中文会报错
    /*
    let z = String::from("中文字符");
    //'byte index 1 is not a char boundary; it is inside '中' (bytes 0..3) of `中文字符`
    let zs1 = &z[0..1];
    */
    
}
