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