// T vs &T
trait Trait {
    fn f(self);
}

impl<T> Trait for fn(T) {
    fn f(self) {
        println!("T");
    }
}

impl<T> Trait for fn(&T) {
    fn f(self) {
        println!("&T");
    }
}

#[test]
fn test_t1() {
    let a: fn(_) = |_: u8| {};
    let b: fn(_) = |_: &u8|{}; // 这里 &u8 整个做为一个T,
    let c: fn(&_) = |_: &u8|{};

    a.f();
    b.f();
    c.f();
}
