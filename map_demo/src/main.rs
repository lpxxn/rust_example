
use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("a".to_string(), 1);
    scores.insert("b".to_string(), 2);

    let keys = vec![String::from("abc"), "cde".to_string()];
    let values = vec![1, 2];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    let k = String::from("abc");
    if let Some(v) = scores.get(&k) {
        println!("v {}", v);
    }

    let v = scores.get(&"abcd".to_string()); // get 返回的是option
    match v {
        Some(expr) => println!("v {}", expr),
        None => println!("None"),
    }

    // 遍历的时候是无序的
    for (k, v) in &scores {
        println!("k {}, v {}", k, v);
    }

    let mut ss = HashMap::new();
    ss.insert("one", 1);
    ss.insert("tow", 2);
    ss.insert("one", 3);
    // 根据旧值更新
    if let Some(v) = ss.get_mut("two") {
        *v = *v + 10;
    }
    // 不存在的时候插入
    ss.entry("three").or_insert(3);
    ss.entry("three").or_insert(55);
    for (k, v) in &ss {
        println!("k {}, v {}", k, v);
    }
    println!("Hello, world!");
}
