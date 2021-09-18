use std::cell::RefCell;

fn main() {
    println!("Hello, world!");
    let mock_msg = MockMsg::new();
    let mut limit = LimitTracker::new(&mock_msg, 5);
    limit.set_value(10);


    for m in mock_msg.send_msg.borrow().iter() {
        println!("msg: {}", m)
    }
    for m in mock_msg.send_msg.into_inner() {
        println!("msg: {}", m)
    }

}

struct MockMsg {
    send_msg: RefCell<Vec<String>>,
}

impl MockMsg {
    fn new() -> MockMsg {
        MockMsg {
            send_msg: RefCell::new(vec![])
        }
    }
}

impl Messenger for MockMsg {
    fn send_msg(&self, msg: &str) {
        self.send_msg.borrow_mut().push(String::from(msg));
    }
}

trait Messenger {
    fn send_msg(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    fn new(m: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger: m,
            value: 0,
            max,
        }
    }
    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send_msg("error you are over your quota")
        } else if percentage_of_max >= 0.9 {
            self.messenger.send_msg("you are used up over 90% of your quota")
        } else {
            self.messenger.send_msg(&format!("you are used up over {}% of your quota", percentage_of_max))
        }
    }
}


