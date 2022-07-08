use std::collections::HashMap;

#[test]
fn test_v() {
    let mut v = vec![1, 2, 3, 4];
    let mut v2 = v.iter_mut().map(|x| *x +10);
    println!("next: {:?}", v2.next());

    let mut v2 = v.iter_mut().map(|x| {*x +=10; (x, 1, "abc")});
    println!("next: {:?}", v2.next());
    let mut v3 = v.iter_mut().map(|x| {*x+=1; (x, 1)});
    println!("v3: {:?}", v3.next());
}