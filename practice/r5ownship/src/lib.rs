// enum 只有包含的元素都是基础元素时，才会实现Copy
#[derive(Debug)]
#[derive(Copy, Clone)]
enum TestEnum {
    //RETYPE(String),
    //        ------ this field does not implement `Copy`
    RETYPE2(u32),
}

#[test]
fn it_works() {
    let a = A;
    let b = a;
    println!("{:?}", a);

    let a = B { a: 1 };
    let b = a;
    println!("{:?}", a);

    let a1 = C { a: "abc".to_string(), b: 1 };
    // 没有用，一样报错
    //let b = a;
    let b = C{
        a: a1.a.clone(),
       ..a1
    };
    let c = a.clone();
    println!("{:?}", a1);
}

// 默认是不实现copy的，要手动加入derive(Copy, Clone)
#[derive(Debug)]
#[derive(Copy, Clone)]
struct A;

#[derive(Debug)]
#[derive(Copy, Clone)]
struct B {
    a: i32,
}

#[derive(Debug)]
//#[derive(Copy, Clone)] // 有堆变量后。加这个也就没有用了
struct C {
    a: String,
//   --------- this field does not implement `Copy`
    b: i32,
}
// 就是自己为struct实现了Copy也不行，只要是数据成员里有没有实现Copy的数据比如String，就
// 没有办法

// 所以，如果有堆内存的变量，加derive(Copy, Clone)也就不好用了。要自己加
impl Clone for C{
    fn clone(&self) -> Self {
        C{
            a:self.a.clone(),
            ..*self
        }
    }
}
// 解注释看编译器提示
//impl Copy for C {}
