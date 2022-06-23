struct Interface<'a> {
    manager: &'a mut Manager<'a>
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("hello")
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface{
            manager: &mut self.manager
        }
    }
}
#[test]
fn test_lifetime() {
    let mut list = List{
        manager: Manager {
            text: "Hello"
        }
    };
    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");
    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完后就应该被释放
    use_list(&list);
    // 由于get_interface的方法把自己的 manager的借给了Interface，但是interface的生命周期和list是一样长。所以
    // 没有被释放掉。
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}