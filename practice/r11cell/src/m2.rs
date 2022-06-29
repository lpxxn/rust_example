use std::cell::RefCell;
use std::rc::Rc;

pub trait Messenger {
    fn send(&self, msg: String);
}

pub struct MsgQueue {
    msg_cache: Rc<RefCell<Vec<String>>>,
}

impl Messenger for MsgQueue {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}

#[test]
fn testMsg() {
    let m = MsgQueue { msg_cache: Rc::new(RefCell::new(vec!["hihihi~~".to_string()])) };
    let m1 = m.msg_cache.clone();
    let m2 = m.msg_cache.clone();
    m.send( "hello".to_string());
    println!("{:?}\n {:?}", m1, m2);
}