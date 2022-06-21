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