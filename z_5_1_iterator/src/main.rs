// 迭代器负责遍历序列中的每一项和决定序列何时结束的逻辑
// 创建迭代器：迭代器是惰性的，意思就是在调用方法使用迭代器之前，不会有任何效果
// 每个迭代器都实现了 iterator trait
/*
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // type Item和Self::Item这种用法叫做定义trait的关联类型
    // next 是Iterator 被要求实现的唯一的方法，next一次返回一个元素，当迭代器结束时，返回None
}
*/

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // 不会对v1产生任何影响
                                 // for v in v1_iter {
                                 //     println!("v: {}", v);
                                 // }
                                 // 这里调用next next 方法是 &mut self。所以上面mut v1_iter
    if let Some(v) = v1_iter.next() {
        println!("v {}", v);
    } else {
        println!("end...")
    }

    let mut iter_next = || {
        if let Some(v) = v1_iter.next() {
            println!("v {}", v);
        } else {
            println!("end...")
        }
    };
    iter_next();
    iter_next();
    iter_next();
    iter_next();

    // change value
    let mut v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter_mut();
    if let Some(v) = v2_iter.next() {
        *v = *v + 3;
    }
    println!("v2 = {:?}", v2);
    // 消费适配器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("total {}", total);
    // 迭代适配器
    let v1 = vec![1, 2, 3];
    println!("v1: {:?}", v1);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    let v2: Vec<_> = v1.into_iter().filter(|x| *x > 5).collect();
    println!("v2: {:?}", v2);

    let mut c = Counter::new();
    for i in (0..6) {
        if let Some(v) = c.next() {
            println!("i {}, v {}", i, v);
        } else {
            println!("i {}, end", i);
            break;
        }
    }
}

/*
_ in a type is a "type placeholder" and has the meaning "infer this type".
Type placeholders are not allowed in item signatures, because global inference is not allowed by design.
Patterns like Vec<_> are called "partial type hints" and are useful in combination with things like collect() on iterators.
Using a _ will let you provide a partial hint:供部分提示

let one_to_one_hundred = (1..101).collect::<Vec<_>>(); This says "Collect into a Vec<T>, please, but infer what the T is for me."
 _ is sometimes called a "type placeholder" for this reason.
*/

#[derive(Debug)]
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
