struct Struct {
    e: i32,
    e2: i32,
    e3: i32
}

fn main() {
    let (a, b, c, d, e, e3);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有是一个变量名而是使用了 _
    [c, .., d, _, _] = [1, 2, 3, 4, 5, 6];
    Struct { e, e3, ..  } = Struct { e: 5, e2: 1, e3: 3 };
    print!("c: {} d: {}, e:{} e3: {}", c, d, e, e3);
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
