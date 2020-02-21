#[derive(Debug)]
struct Dog {
    name: String,
    count: i32,
}

impl Drop for Dog {
    fn drop(&mut self) {
        println!("Dog name: {} leave", self.name);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let a = super::Dog{name:"wangcai".to_string(), count: 2};
        {
            let a = super::Dog{name:"dahuang".to_string(), count: 2};
        }
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn eary_drop() {
        // 提前释放
        let a = super::Dog{name:"wangcai".to_string(), count: 2};
        let b = super::Dog{name:"dahuang".to_string(), count: 2};
        drop(b);
        println!("------------");
    }
}
