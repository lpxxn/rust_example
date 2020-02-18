
fn main() {
    // 绝对路径
    mylib::factory::produce_refrigerator::produce_re();

    // 使用 use
    use mylib::factory::produce_refrigerator;
    produce_refrigerator::produce_re(); 

    use mylib::factory::produce_refrigerator::produce_re;
    produce_re(); 


    use mylib::factory::produce_washing_machine as a;
    a::produce_wa();

    use mylib::factory::*;
    produce_refrigerator::produce_re(); 
    produce_washing_machine::modB::print_mod_b();

    use mylib::entity::*;
    let a = A::new_a("abc".to_string(), 1);
    println!("age: {}", &a.age());
    a.print_a();
    println!("Hello, world!");
}
