fn main() {
    println!("Hello, world!");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}", i);
    #[derive(Debug)]
    struct Point {
        a: i32,
        b: i32,
    }

    let p1 = Point{a: 1, b: 2};
    let p2 = Point{
        ..p1
    };
    println!("p2 {:?}", p2);
    let p3 = Point {
        b: 5,
        ..p1
    };
    println!("p3 {:?}", p3);
    let s: SomeOptions = Default::default();
    println!("s: {:?}", s);
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

trait A {
    fn print(&self);
}
#[derive(Default, Debug)]
struct SomeOptions {
    i: i32,
    s: String,
    v: Vec<i32>,
    oS: Option<String>,
    // o: Option<T>,
    // b: Box<dyn A>,
}
