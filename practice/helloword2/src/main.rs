fn main() {
    println!("Hello, world!");
    let s = String::from("hello");
    let slice = &s[..2];
    let slice = &s[0..2];
    let slice = &s[..s.len()];
    // \ 用来转义
    println!("{}", "hello \x52\x75\x73\x74");
    // 不想转义
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    // 用r
    let raw_str = r"hello \x52\x75\x73\x74";
    println!("{}", raw_str);
    // 如果有 " 可以加 # 并以## 结束
    println!("{}", r#"你好"你好""#);
    // 如果还有歧义,比如字符串内有相同数量的#，可以继续加 #
    println!("{}", r###"abcd # deffe"##"###);
    testString();
}

fn testString() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; // 使用两种方法
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");
 
    //Note! The type here cant be `&mut str` due to `push` is ONLY defined on String type and its mut reference: `&mut String` !
    // So you can't use `s.as_mut_str()`
    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
}
