pub mod add_one_fn;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one_fn::add_one(3), 4);
    }
}
