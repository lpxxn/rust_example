fn main() {
    let v1 = vec![1, 2, 3, 5, 2];
    let l = largest(&v1);
    println!("l {}", l);

    let v2 = vec!['a', 'b', 'z', 's'];
    let l = largest(&v2);
    println!("l {}", l);

    let p1 = Pointer{x: 1, y: 2};
    println!("p1 {:#?}", p1);
    println!("p1 x {}, y {}", p1.get_x(), p1.get_y());

    let p2 = Point2{x: 1, y: 'a'};
    println!("p2 {:?}", p2);
    println!("p1 x {}, y {}", p2.get_x(), p2.get_y());
    let p23 = Point2{x: 1.2, y: "中国"};
    let p24 = p23.creat_point(p2);
    println!("p24 {:?}", p24);
    /*
    let p24 = p23.creat_point(p2);
   |                            -- value moved here
     因为creat_point 方法是self
     value borrowed here after move
    */
    // println!("p2 {:?}", p2);

    let p31 = Point3{x:4, y: 'a'};
    let p32 = Point3{x:1.1, y: (1, 2)};
    let p33 = p32.creat_point(&p31);
    println!("p33 {:?}", p33);
    println!("p31 {:?}", p31);
    println!("Hello, world!");
}
    // 栈
    // 栈上的数据只有一份。 赋值操作相当于深拷贝.
    // 椎上的数据由栈上的指针指向椎上的内容，默认情况下赋值操作是浅拷贝 深拷贝用clone()
    // 基本的数据类型 bool i32 char 元组等实现了 copy trait 在赋值操作后还可以使用
    // PartialOrd 是可对比的 trait
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

#[derive(Debug)]
struct Pointer<T> {
    x: T,
    y: T,
}

impl<T> Pointer<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &U {
        &self.y
    }

    // 这个 self 没有&
    fn creat_point<V, W>(self, other: Point2<V, W>) -> Point2<V, T> {
        Point2 {
            x: other.x,
            y: self.x,
        }
    }
}

// 枚举
enum TResult<T, E> {
    OK(T),
    Err(E),
}

#[derive(Debug)]
struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T: Copy, U: Copy> Point3<T, U> {
       // 这个 self 有&
       fn creat_point<V, W: Copy>(&self, other: &Point3<V, W>) -> Point3<W, T> {
        Point3 {
            x: other.y,
            y: self.x,
        }
    } 
}

struct Foo<T: ?Sized> {
    a: Box<T>,
}
struct Foo2 {
    fo: Foo<Fn(i32)>
}
/*
可以看到编译器提示:Fn(i32)没有实现Sized
这个要从编译器的默认行为说起
1. 编译器默认struct Foo<T>这种定义都要求T是Sized，也就是这个类型有固定的长度，比如i32，u64之类的
2. 有几种类型长度不固定，其中之一就是trait类型，因为可能存在多种类型都可以实现某个trait，这样把trait作为类型T的时候编译器无法知道可能传递过来的实现这个trait的类型的size，从另一个意义上来说实现某个trait的类型的长度不是固定的。
3. 我们这儿把Fn(i32)作为类型T传递给Foo，很明显可能存在很多种类型可以实现Fn(i32)这个trait，所以Fn(i32)长度并不固定。但从1来看编译器默认是认为Foo是需要一个Sized的类型T

解决这个问题其实很简单，因为Foo里的a,b都是保存的Box<T>也就是说T的长度是否固定并不重要，所以我们不需要编译器默认给我们强加的T必须Sized这个规定，修改定义即可：struct Foo<T:?Sized>这也就是告诉编译器Foo<T>可以接收Sized或者不是Sized的类型都可以，这样Fn(i32)就可以了
————————————————
版权声明：本文为CSDN博主「varding」的原创文章，遵循 CC 4.0 BY-SA 版权协议，转载请附上原文出处链接及本声明。
原文链接：https://blog.csdn.net/varding/article/details/48326341
*/