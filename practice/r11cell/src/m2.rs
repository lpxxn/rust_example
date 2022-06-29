use std::cell::{Cell, RefCell};
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


fn is_even(i: i32) -> bool {
    i % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    // for num in nums.iter().filter(|num| is_even(**num)) {
    //     nums[i] = *num;
    //     i += 1;
    // }
    /*
    34 |     for num in nums.iter().filter(|num| is_even(**num)) {
   |                ----------------------------------------
   |                |
   |                immutable borrow occurs here
   |                immutable borrow later used here
35 |         nums[i] = *num;
   |         ^^^^ mutable borrow occurs here
     */
    let slice:&[Cell<i32>] = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
    for num in slice.iter().filter(|num| is_even(num.get())) {
        slice[i].set(num.get());
        i+=1;
    }
    nums.truncate(i);
}

fn retain_even2(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i+=1;
        }
    }
    nums.truncate(i);
}

#[test]
fn testEven2() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    retain_even(&mut nums);
    println!("{:?}", nums);

    // test 3
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    retain_even2(&mut nums);
    println!("{:?}", nums);
}
