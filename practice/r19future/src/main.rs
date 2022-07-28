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

/// Spawner 负责创建新的 Future 然后将他发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static +Send) {
        let future = future.boxed();
        let task = Arc::new(Task{
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone()
        });
        self.task_sender.send(task).expect("任务队列已满")
    }
}

struct Task {
    /// 进行中的Future，在未来的某个时间点会完成
    ///
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}


fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}


fn main() {
    println!("Hello, world!");
}
