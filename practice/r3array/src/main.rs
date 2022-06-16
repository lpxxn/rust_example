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

    for v in &a2 {
        println!("{}", v);
    }
    println!("index: {}", a2[2]);

}
