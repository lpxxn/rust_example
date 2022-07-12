// trait 默认是static的

trait Foo<'a> {}

struct FooImpl<'a> {
    s: &'a [u32],
}

impl<'a> Foo<'a> for FooImpl<'a> {}

// 'longest: 'short
// 输入要比输出的生命周期长
fn foo<'a: 'b, 'b>(s: &'a [u32]) -> Box<dyn Foo<'b> + 'b> {
    Box::new(FooImpl { s })
}

fn foo2<'a>(s: &'a [u32]) -> Box< dyn Foo<'a> + 'a> {
    Box::new(FooImpl { s })
}

/*
 Box 默认是 'static 相当于  Box<dyn Foo<'a> + 'static>
17 | fn foo2<'a>(s: &'a [u32]) -> Box< dyn Foo<'a> > {
   |                --------- this data with lifetime `'a`...
18 |     Box::new(FooImpl { s })
   |              ^^^^^^^   - ...is used and required to live as long as `'static` here
 */

#[test]
fn test_trait() {

}