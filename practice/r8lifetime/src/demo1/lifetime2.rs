use std::fmt::Debug;

struct Interface<'a: 'b, 'b> {
    manager: &'b mut Manager<'a>
}

impl<'a: 'b, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consume")
    }
}

struct Manager<'a> {
    text: &'a str
}

// 我们需要为 get_interface 方法的参数给予一个不同于 List<'a> 的生命周期 'b
struct List<'a> {
    manager: Manager<'a>
}

impl<'a> List<'a> {
    // 也可以说是给 Interface 一个小于'a 的生命周期
    pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b>
        where 'a: 'b {
        Interface {
            manager: &mut self.manager
        }
    }
}

#[test]
fn test_lifetime() {
    let mut list = List {
        manager: Manager{
            text: "hello"
        }
    };
    list.get_interface().noop();

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text)
}

/*
https://course.rs/advance/lifetime/basic.html#%E4%B8%89%E6%9D%A1%E6%B6%88%E9%99%A4%E8%A7%84%E5%88%99
三条消除规则
编译器使用三条消除规则来确定哪些场景不需要显式地去标注生命周期。其中第一条规则应用在输入生命周期上，第二、三条应用在输出生命周期上。若编译器发现三条规则都不适用时，就会报错，提示你需要手动标注生命周期。

每一个引用参数都会获得独自的生命周期

例如一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。

若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期

例如函数 fn foo(x: &i32) -> &i32，x 参数的生命周期会被自动赋给返回值 &i32，因此该函数等同于 fn foo<'a>(x: &'a i32) -> &'a i32

若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升。

规则其实很好理解，但是，爱思考的读者肯定要发问了，例如第三条规则，若一个方法，它的返回值的生命周期就是跟参数 &self 的不一样怎么办？总不能强迫我返回的值总是和 &self 活得一样久吧？! 问得好，答案很简单：手动标注生命周期，因为这些规则只是编译器发现你没有标注生命周期时默认去使用的，当你标注生命周期后，编译器自然会乖乖听你的话。

https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
https://practice.rs/lifetime/static.html#t-static

 */

/*
这个生命周期并不一定是指全局生存期，它的意思是
“就算眼前这个scope里的所有其他东西都死了它也还活着所以你就可以放心的使用它而不用担心悬空引用了
 */
fn print_it<T: Debug +'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2(input: impl Debug +'static) {
    println!("'static value passed in is:{:?}", input);
}

// fn print_it3<'a, T>(input: T) where for<'b> T: Debug + 'a {
//     println!("'static value passed in is: {:?}", input);
// }
fn print_it3<T>(input: T) where T: Debug {
    println!("'static value passed in is: {:?}", input);
}
fn print_it4<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}
#[test]
fn test_print() {
    let i = 5;
    print_it(i);
    print_it2(i);

    //print_it(&i);
    //print_it2(&i);

    print_it3(&i);
    print_it2(i);
    print_it4(&i);/*
    这段代码竟然不报错了！原因在于我们约束的是 T，但是使用的却是它的引用 &T，
    换而言之，我们根本没有直接使用 T，因此编译器就没有去检查 T 的生命周期约束！
    它只要确保 &T 的生命周期符合规则即可，在上面代码中，它自然是符合的。
    */
}

/*
关于Bounds：

T: 'a  //T里的所有引用，都可以在'a生命周期之外存活
T: Trait+'a //T实现Trait特质，且T内的所有引用，都在'a生命周期之外存活。
 */
// Ref是一个tuple，'a是生命周期，'a生命周期比tuple更长
// tuple只有一个元素，这个元素是一个引用，是T的引用，这个引用的生命周期比Ref更长
// T内部的所有引用都在生命周期'a，都超过Ref。
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

// 一个打印的范型函数。它的入参是范型T，且范型T必须实现Debug特质。
// 因为实现了Debug特质，这个打印函数根据Debug特质进行打印。
// 这个where指示T的属性
fn print<T>(t: T) where T: Debug {
    println!("`print`: t is {:?}", t);
}

// 这个print_ref函数，跟上面的print函数不同之处：
// 这里的入参是引用，因此必须有生命周期。
// 生命周期的意思是，T的生命周期是'a，比print_ref生命周期更长，T的所有引用都在'a
// +'a 表示 a生命周期一定在print_ref外头
fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}


struct Ref2<'a, T>(&'a T); // 等价于 struct Ref<'a, T: 'a>(&'a T);
#[test]
fn test_print2() {
    static STATIC_S: String = String::new();
    let s = String::new();

    let _: Ref2<'_, String> = Ref2(&s);
    let r2 = Ref2(&s);
    let _: Ref2<'static, String> = Ref2(&STATIC_S);
    //let _: Ref2<'static, String> = Ref2(&s);
}