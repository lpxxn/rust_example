// 高阶生命周期， 高阶 trait 限定

use std::fmt::Debug;

fn print_it<T: Debug>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

#[test]
fn test_fn() {
    let s = 1;
    print_it(&s);
}


#[test]
fn test_fn2() {
    //let f = |x: &i32| x;
    // late bound
    // let f: for<'a> Fn(&'a i32) -> &'a i32 = |x| x; // 不支持

    let f = annotate(|x: &i32| x);
    let i = &3;
    let j = f(i);

    let f2 = annotate2(|x: &i32| x);
    let j = f2(i);
}
// late bound
fn annotate<T, F>(f: F) -> F
    where for<'a> F: Fn(&'a T) -> &'a T {
    f
}

// early bound
fn annotate2<'a, T: 'a, F>(f: F) -> F where F: Fn(&'a T) -> &'a T {
    f
}
