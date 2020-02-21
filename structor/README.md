https://learning-rust.github.io/docs/d3.modules.html

```
// 01. Calling private functions of the same module
fn main() {
  phrases::greet();
}

mod phrases {
  pub fn greet() {
    hello(); // Or `self::hello();`
  }

  fn hello() {
    println!("Hello, world!");
  }
}

// 02. Calling private functions of the parent module
fn main() {
  phrases::greetings::hello();
}

mod phrases {
  fn private_fn() {
    println!("Hello, world!");
  }

  pub mod greetings {
    pub fn hello() {
      super::private_fn();
    }
  }
}
```

The self keyword is used to refer the same module, while the super keyword is used to refer parent module. Also, the super keyword can be used to access root functions from inside a module.


##  In a different file, same directory
```
// ↳ main.rs
mod phrases;

fn main() {
  phrases::greetings::hello();
}

// ↳ phrases.rs
pub mod greetings { // ⭐️ The module has to be public to access from outside
  pub fn hello() {
    println!("Hello, world!");
  }
}
```

##  In a different file, different directory

mod.rs in the directory module root is the entry point to the directory module. All other files in that directory root, act as sub-modules of the directory module.

Again, If we wrap file content with a mod declaration, it will act as a nested module.

```
// ↳ main.rs
mod phrases;

fn main() {
  phrases::greetings::hello();
}

// ↳ phrases/mod.rs
pub mod greetings { // ⭐️ The module has to be public to access from outside
  pub fn hello() {
    println!("Hello, world!");
  }
}
```