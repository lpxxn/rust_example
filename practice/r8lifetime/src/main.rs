fn main() {
    let s1 = "hello".to_string();
    let mut i1 = ImportantExcerpt{part: &s1[..]};
    let mut result : &str;
    {
        result = i1.announce_and_return_part("abc");
    }

    println!("result: {}", result);
    {
        result = i1.announce_and_return_part2("haha");
    }
    println!("result: {}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}
// 'a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
//
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
    // 可以把 'a 和 'b 都在同一个地方声明（如上），或者分开声明但通过 where 'a: 'b 约束生命周期关系，如下：
    fn announce_and_return_part2<'c>(&'a self, announcement: &'c str) -> &'c str
        where 'a: 'c {
        println!("Attention please: {}", announcement);
        self.part
    }
}

