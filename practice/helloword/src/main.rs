
fn main() {
    println!("Hello, world!");
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    println!("a: {} a bit: {:x}", abc.1, abc.1.to_bits());

    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    let x = "你好，世界";
    for c in x.chars() {
        println!("{}", c);
    }

    let x = "holla中国人नमस्ते";
    for c in x.chars() {
        println!("{}", c);
    }
    // 用这个库 得到子串信息 https://crates.io/crates/utf8_slice
}
