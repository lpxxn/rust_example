fn main() {
    let  a1: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", a1);

    for v in &a1 {
        println!("{}", v);
    }
    println!("index: {}", a1[2]);
    let mut a2: [String; 5] = ["a".to_string(), "b".to_string(), "c123".to_string(), "d".to_string(), "e".to_string()];
    println!("{:?}", a2);
    for v in a2.iter_mut() {
        v.push_str("aaaaa");
        println!("{}", v);
    }

    for v in &mut a2 {
        *v = v.replace("aaa", "bbb");
    }
    println!("---------");
    for v in &a2 {
        println!("{}", v);
    }
    println!("index: {}", a2[2]);

}

/*
使用方法	                   等价使用方式	                                        所有权
for item in collection	     for item in IntoIterator::into_iter(collection)	转移所有权
for item in &collection	     for item in collection.iter()	                    不可变借用
for item in &mut collection	 for item in collection.iter_mut()	                可变借用
*/