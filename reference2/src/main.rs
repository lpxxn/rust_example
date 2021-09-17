use std::ops::Deref;

fn main() {
    println!("Hello, world!");
    let s = "hi".to_string();  // : String
    let b = &*s;   // equivalent to `&(*s)`
    let c = &(*s);

    let d = std::ops::Deref::deref(&s);
    let x = 5;

    let y = &x;
    let y1 = *y;
    let z = *std::ops::Deref::deref(&y);
    /*
    So, *s is actually *std::ops::Deref::deref(&s), in which the deref() function has the return type &str which is then dereferenced again. Thus, *s has the type str (note the lack of &).
    In general, &* means to first dereference (*) and then reference (&) a value. In many cases, this would be silly, as we'd end up at the same thing.
     */
    let a = A;
    let b = a.get_b();

    let a = Box::new(A);
    let b = a.get_b();
    std::mem::drop(a);
    // a和b的生命周期一样长，如果 drop掉，也就不能用b了
    //let c = b;
}

struct A;
struct B;
impl Deref for A {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        let x = &B;
        return x
    }
}

fn foo(a: A) -> Option<()> {
    let v = &&a;
    bar(v);
    bar(&&a);
    bar((&a).deref());
    bar(&*(&a));


    let a1 = &a;
    bar(a1.deref());
    Some(())
}

fn bar(b: &B) {

}

impl A {
    fn get_b(&self)  -> &B {
        &B
    }
}

// fn get_b() -> &B {
//     &B
// }