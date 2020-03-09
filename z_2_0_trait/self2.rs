/*
The one exception is the implicit Self type of a trait. A trait does not have an implicit Sized bound as this is incompatible with trait objects where, by definition, the trait needs to work with all possible implementors, and thus could be any size.
Although Rust will let you bind Sized to a trait, you won't be able to use it to form a trait object later:
*/

fn main() {
    trait Bar2<'a> {
        fn my(&self) -> &Self
        where
            Self: Sized;
        fn my2(&mut self) -> &mut dyn Bar2<'a>;
        fn set_next(&mut self, n: &'a dyn Bar2<'a>);
        fn p(&self) {
            println!("hello");
        }
    }

    struct Impl2<'a> {
        next: Option<&'a dyn Bar2<'a>>,
    };
    impl<'a> Bar2<'a> for Impl2<'a> {
        fn my(&self) -> &Self {
            self
        }
        fn my2(&mut self) -> &mut dyn Bar2<'a> {
            self
        }
        fn set_next(&mut self, n: &'a dyn Bar2<'a>) {
            self.next = Some(n);
        }
    }
    impl<'a> Impl2<'a> {
        fn new() -> Impl2<'a> {
            Impl2 { next: None }
        }
    }

    let mut p1 = Impl2::new();
    let mut p2 = Impl2::new();
    //let i: () =  &p2;
    let y: &mut dyn Bar2 = &mut p2;
    y.p();
    //let z: &dyn Bar2 = y.my();
    let z2: &mut dyn Bar2 = y.my2();
    // To fix this error, ensure that you don't have any other references to the
    // variable before trying to access it mutably:
    //p1.set_next(&p2);

    //z2.my();
    z2.my2();
    z2.p();

    p1.set_next(z2);
}
// https://doc.rust-lang.org/std/marker/trait.Sized.html
