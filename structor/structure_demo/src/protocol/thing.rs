#[derive(Debug)]
pub struct Thing(i8);

impl Thing {
    pub fn new() -> Thing {
        Thing(5)
    }
}
