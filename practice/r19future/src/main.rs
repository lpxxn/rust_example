use std::io::Take;
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    // 引入自己的lib
    timer_future::TimerFuture,
};

/// 任务执行器，负责从通道中接收任务后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}



fn main() {
    println!("Hello, world!");
}
