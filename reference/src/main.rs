fn main() {
    let mut str1 = String::from("hello");
    let l = ref_test1(&str1);
    println!("str1 = {}", str1);
    println!("len = {}", l);
    str_add(&mut str1);
    println!("str1 = {}", str1);
}

fn ref_test1(s: &String) -> usize {
    s.len()
}

fn str_add(s: &mut String) {
    s.push_str("abcdef end.")
}

// 引用 & 指向变量的地址，并不拥有它，所以引用离开其值指向的作用域后也不会被
// 丢弃
// 借用 &mut