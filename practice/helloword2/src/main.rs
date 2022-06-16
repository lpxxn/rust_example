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
    test_string();
}

fn test_string() {
    let mut s = String::from("hello, world");

    let slice1: &str = &s; // 使用两种方法
    assert_eq!(slice1, "hello, world");
 
    let st1: &str = "abcdefr";

    let s2 = &s[..2];
    let slice2 = &s[..5];
    assert_eq!(slice2, "hello");
 
    //Note! The type here cant be `&mut str` due to `push` is ONLY defined on String type and its mut reference: `&mut String` !
    // So you can't use `s.as_mut_str()`
    let slice3: &mut String = &mut s;
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
}

/* String 不能用 index
 let s1 = String::from("hello");
   let h = s1[0];
 3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`

字符串的底层的数据存储格式实际上是[ u8 ]，一个字节数组。对于 let hello = String::from("Hola"); 这行代码来说，Hola 的长度是 4 个字节，因为 "Hola" 中的每个字母在 UTF-8 编码中仅占用 1 个字节，但是对于下面的代码呢？
let hello = String::from("中国人");
如果问你该字符串多长，你可能会说 3，但是实际上是 9 个字节的长度，因为大部分常用汉字在 UTF-8 中的长度是 3 个字节，因此这种情况下对 hello 进行索引，访问 &hello[0] 没有任何意义，因为你取不到 中 这个字符，而是取到了这个字符三个字节中的第一个字节，这是一个非常奇怪而且难以理解的返回值。
  
 */