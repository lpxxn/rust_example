
#[test]
fn test_vec() {
    let mut v = vec![];
    //let vv = &v;
    {
        v.push(1);
        v.push(2);
    }
    let vv = &v;
    vv;
    println!("v: {:?}", v);
}

// late bound vs early bound
// 泛型和函数生命周期，都是late bound
#[test]
fn test_struct() {
    #[derive(Debug)]
    struct A<T>(T);
    let a = A::<i32>(3);
    println!("a: {:?}", a);


    let s1 = "rust".to_string();
    let s1_r = &s1;
    {
        let s2 = "c".to_string();
        let res = the_longest(s1_r, &s2);
        println!("{} is the longest", res);
    }
}
fn the_longest<'c, 'a: 'c, 'b: 'c>(s1: &'a str, s2: &'b str) -> &'c str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}