pub fn add_two(i: i32) -> i32 {
    i + 2
}

#[derive(Debug)]
pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl AverCollect {
    pub fn new() -> AverCollect {
        AverCollect {
            list: vec![],
            aver: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        if let Some(rev) = self.list.pop() {
            self.update_average();
            return Some(rev)
        }
        None
    }
    pub fn average(&self) -> f64 {
        self.aver
    }
    
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}
