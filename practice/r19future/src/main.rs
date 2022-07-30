use std::io::Take;
use futures::task::waker;
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

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            // 获取一个future，若它还没有完成(仍然是Some，不是None)，则对它进行一次poll并尝试完成它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                // 基于你的血自身创建一个 LocalWaker
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                // `BoxFuture<T>`是`Pin<Box<dyn Future<Output = T> + Send + 'static>>`的类型别名
                // 通过调用`as_mut`方法，可以将上面的类型转换成`Pin<&mut dyn Future + Send + 'static>`
                if future.as_mut().poll(context).is_pending() {
                    // Future 还没执行完，因此将他放回任务中，等待下次被poll
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// Spawner 负责创建新的 Future 然后将他发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满 new spawn")
    }
}

struct Task {
    /// 进行中的Future，在未来的某个时间点会完成
    ///
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将任务自身放回到任务通道中，等待执行器的poll
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满 in task")
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

    (Executor { ready_queue }, Spawner { task_sender })
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    // 生成一个任务
    spawner.spawn(async {
        println!("howdy !");
        // 创建定时任务Future, 并等待他完成
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    // drop掉任务，这样执行器就知道任务已经完成，不会再有新的任务进来
    drop(spawner);
    // 运行执行器直到任务队列为空
    // 任务运行后，会先打印`howdy!`, 暂停2秒，接着打印 `done!`
    executor.run();
}
