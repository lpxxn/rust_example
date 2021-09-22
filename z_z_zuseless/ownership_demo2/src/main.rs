fn main() {
    let mut s1: Option<String> = Some(String::from("hello"));
    for _ in 1..1 {
        s1 = {
            match s1 {
                Some(s) => {
                    Some(String::from(s))
                }
                _ => {
                    None
                }
            }
        }
    }

    // 如果上面没有对s1重新赋值会有error
    // error borrow of moved value
    if let Some(ref s) = s1 {
        println!("{}", s);
    }
}
