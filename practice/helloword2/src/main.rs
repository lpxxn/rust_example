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
}
