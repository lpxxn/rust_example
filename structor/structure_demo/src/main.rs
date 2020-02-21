mod protocol;
fn main() {
    let a = protocol::Thing::new();
    println!("hello, {:?}", a);
    phrases::greetings::hello();
    phrases::greetings::greet();
}

mod phrases {
    fn private_fn() {
        println!("Hello, private_fn!");
    }

    pub mod greetings {
        pub fn hello() {
            println!("Hello, world!~~~");
        }
        pub fn greet() {
            hello(); // Or `self::hello();`
            super::private_fn();
        }
    }
}
/*
The `self` keyword is used to refer the same module, while the super keyword is used to refer parent module. Also, 
the `super` keyword can be used to access root functions from inside a module.
*/