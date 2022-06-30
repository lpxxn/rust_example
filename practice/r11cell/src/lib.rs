mod m2;
mod circle_ref1;

use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

struct MsgQueue {
    msg_cache:RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: &str) {
        self.msg_cache.borrow_mut().push(String::from(msg));
    }
}


fn send_message(m: &dyn Messenger, msg: &str) {
    m.send(msg);
}

#[test]
fn it_works() {
    let m = MsgQueue { msg_cache: RefCell::new(Vec::new()) };
    send_message(&m, "hello");
}
