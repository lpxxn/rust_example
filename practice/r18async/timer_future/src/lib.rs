use std::{future::Future, pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Wake},
        thread,
        time::Duration,
};
use std::task::Waker;

/// 在Future 和等待的线程间共享状态
struct SharedState {
    /// 定制（睡眠）是否结束
    completed: bool,
    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}




#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
