use std::ops::Add;
use crate::d1::t1::test_pair::Pair;

pub mod test_pair {
    use std::cmp::Ordering;
    use std::fmt::{Display, Formatter};
    use std::ops::Add;
    use std::process::Output;

    pub struct Pair<T> {
        x: T,
        y: T
    }

    impl<T> Pair<T> {
       pub fn new(x: T, y: T) -> Self {
            Pair {
                x,y
            }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        // 只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法。 该函数可读性会更好
        pub fn cmp_display(&self) {
            if self.x >= self.y {
                println!("largest is x {}", self.x)
            } else {
                println!("largest is y {}", self.y)
            }
        }
    }

    impl PartialOrd for Pair<i32> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.x.partial_cmp(&other.x)
        }
    }

    impl Display for Pair<i32> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "(x: {}, y: {})", self.x, self.y)
        }
    }

    impl PartialEq<Self> for Pair<i32> {
        fn eq(&self, other: &Self) -> bool {
            return self.x == other.x && self.y == self.y
        }
    }

    impl<T: Add<T, Output = T>> Add for Pair<T> {
        type Output = Pair<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Pair{x: self.x + rhs.x, y: self.y + rhs.y}
        }
    }
}

#[test]
fn test_pari() {
    let p1 = test_pair::Pair::new(1, 2);
    p1.cmp_display()
}

#[test]
fn test_hello() {
    //use std::convert::TryInto;
    let a: i32 = 10;
    let b: u16 = 100;
    let _b = b.try_into().unwrap();
    if a < _b {
        println!("ten is less than on hundred.")
    }
}

#[test]
fn test_add() {
    fn add<T: Add<T, Output= T>>(a: T, b: T) -> T{
        a + b
    }
    let p1 = Pair::new(1, 2);
    let p2 = Pair::new(3, 4);
    println!("add: {}", add(p1, p2));
    println!("add: {}", add(1, 2));
}
// cargo test --bin r7trait d1::t1::test_add -- --show-output
#[test]
fn test_foo() {
    use std::ops;
    struct Foo;
    struct Bar;
    #[derive(PartialEq, Debug)]
    struct FooBar;

    #[derive(PartialEq, Debug)]
    struct BarFoo;
    // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
}