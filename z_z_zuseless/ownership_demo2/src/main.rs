fn main() {
    let mut s1: Option<String> = Some(String::from("hello"));
    for _ in 1..=1 {
        s1 = {
            let mut new_s :Option<String> = None;
            // 因为已经给s1赋值了，所以这里ref 或者 &s1也没啥用了
            if let Some(s) = s1 {
                println!("{}", s);
                new_s = Some(format!("{} world", s))
            }
            new_s
        }
    }

    // 如果上面没有对s1重新赋值会有error
    // error borrow of moved value
    if let Some(ref s) = s1 {
        println!("{}", s);
    }
}
