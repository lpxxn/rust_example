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


 */