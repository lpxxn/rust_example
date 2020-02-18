
fn main() {
    factory::produce_refrigerator::produce_re();
    println!("Hello, world!");
}

// 默认的成员都是私有的 使用 pub 成公有
mod factory {
    pub mod produce_refrigerator {
        pub fn produce_re() {
            println!("produce refrigerator");
        }
    }

    mod produce_washing_machine {
        fn produce_wa() {
            println!("prodece washing machine!");
        }
    }
}