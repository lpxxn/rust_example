// 默认的成员都是私有的 使用 pub 成公有
pub mod produce_refrigerator {
    pub fn produce_re() {
        println!("produce refrigerator");
    }
}

pub mod produce_washing_machine {
    pub fn produce_wa() {
        println!("prodece washing machine!");
    }

    pub mod modB {
        pub fn print_mod_b() {
            println!("mod b");
            super::produce_wa();
        }
    }
}
