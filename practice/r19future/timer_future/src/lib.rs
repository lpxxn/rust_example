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

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState{ completed: false, waker: None }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器，定时器已经完成，可以继续`pool`对应的`Future`了
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });
        TimerFuture{ shared_state }
    }
}



#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
