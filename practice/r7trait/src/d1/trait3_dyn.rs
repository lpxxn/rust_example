trait MyTrait1{
    fn f(&self) -> Self;
}

impl MyTrait1 for u32 {
    fn f(&self) -> Self {
        println!("u32");
       1
    }
}

impl MyTrait1 for String{
    fn f(&self) -> Self {
        println!("string");
       self.clone()
    }
}

fn my_func1(x: impl MyTrait1) -> impl MyTrait1 {
    x.f()
}

#[test]
fn test1() {
    my_func1(12_u32);
    my_func1("abc".to_string());
}


trait MyTrait2 {
    fn f(&self) -> Box<dyn MyTrait2>;
}

impl MyTrait2 for u32 {
    fn f(&self) -> Box<dyn MyTrait2> {
        println!("u32");
        Box::new(1)
    }
}

impl MyTrait2 for String {
    fn f(&self) -> Box<dyn MyTrait2> {
       println!("string");
        Box::new(self.clone())
    }
}

fn my_func2(x: Box<dyn MyTrait2>) -> Box<dyn MyTrait2> {
    x.f()
}

#[test]
fn test2() {
    my_func2(Box::new(123));
    my_func2(Box::new("abc".to_string()));
}