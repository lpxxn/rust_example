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