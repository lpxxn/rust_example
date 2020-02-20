//
fn main() {
    let use_clusure = || {
        println!("this is a closure");
    };
    use_clusure();
    let add_one = |x: i32, y: i32| -> i32 { x + y + 5 };
    println!("add: {}", add_one(1, 2));
    let add_v1 = |x, y| x + y;
    let add_v2 = |x, y| x + y;
    println!("add v2: {}", add_v1(1, 2));
    println!("add v2: {}", add_v2(1, 2));
    // 类型推导，不能推导两次；
    let ex1 = |x| x;
    println!("ex {}", ex1(1));
    // err
    //println!("ex {}", ex1(123.to_string()));
    println!("Hello, world!");

    let a = 10;
    let eg2 = |x| x + a;
    println!("eg2 {}", eg2(1));

    let mut c = Cacher::new(|x| x+ 1);
    let v1 = c.value(1);
    println!("v1 {}", v1);
    let v1 = c.value(81);
    println!("v1 {}", v1);
}

#[derive(Debug)]
struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    calcuation: T,
    value: Option<i32>,
}

impl<T: Fn(i32) -> i32> Cacher<T> {
    fn new(calcuation: T) -> Cacher<T> {
        Cacher {
            calcuation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calcuation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
