pub mod factory;
pub mod entity;

// cargo test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_new_a() {
        use crate::entity;
        let a = entity::A::new_a("name".to_string(), 10);
        assert_eq!(a.age(), 10);
    }
}
