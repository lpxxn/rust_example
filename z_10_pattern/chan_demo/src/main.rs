fn main() {
    let f1 = Officer::new(1);
    let mut f2 = Officer::new(2);
    let mut f3 = Officer::new(3);
    let n1 = f2.set_next(&f1);
    let n2 = f3.set_next(n1);

    n2.handle();

    println!("Hello, world!");
}

struct Officer<'a> {
    v: i32,
    next: Option<&'a Officer<'a>>
}

impl<'a> Officer<'a> {
    fn new(v: i32) -> Officer<'a> {
        Officer{
            v,
            next: None,
        }
    }
    fn set_next(&mut self, next: &'a Officer<'a>) -> &mut Self {
        self.next = Some(next);
        self
    }
    fn handle(&self) {
        println!("v: {}", self.v);
        if let Some(n) = self.next {
            n.handle();
        }
    }
}