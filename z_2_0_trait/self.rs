
/*
The one exception is the implicit Self type of a trait. A trait does not have an implicit Sized bound as this is incompatible with trait objects where, by definition, the trait needs to work with all possible implementors, and thus could be any size.
Although Rust will let you bind Sized to a trait, you won't be able to use it to form a trait object later:
*/

fn main () {
    trait Foo { }
    trait Bar: Sized { }

    trait Bar2 {
        fn my(&self) -> &Self where Self: Sized;
        fn my2(&mut self) -> &mut dyn Bar2;
        fn p(&self) {
            println!("hello");
        }
    }
    
    struct Impl;
    impl Foo for Impl { }
    impl Bar for Impl { }

    struct Impl2<'a> {
        next: Option<&'a dyn Bar2>
    }
    impl<'a> Bar2 for Impl2<'a> {
        fn my(&self) -> &Self    {
            self
        }
        fn my2(&mut self) -> &mut dyn Bar2 {
            self
        }
    }
    impl<'a> Impl2<'a> {
        fn new () -> Impl2<'a> {
            Impl2{next: None,}
        }
        fn set_next(&mut self, n: &'a dyn Bar2) {
            self.next = Some(n);
        }
    }
    
    let x: &dyn Foo = &Impl;    // OK
    //let y: &dyn Bar = &Impl; // error: the trait `Bar` cannot
                                // be made into an object

    let mut p1 = Impl2::new();                                
    let mut p2 = Impl2::new();
    //let i: () =  &p2;
    let y: &mut dyn Bar2 = &mut p2;   
    y.p();
    //let z: &dyn Bar2 = y.my();                         
    let z2: &mut dyn Bar2 = y.my2();
    p1.set_next(z2);
    //z2.my();
    z2.my2();
    z2.p();
}
// https://doc.rust-lang.org/std/marker/trait.Sized.html