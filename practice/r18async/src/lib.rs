use std::thread;
use std::time::Duration;
use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    //thread::sleep(Duration::from_secs(10));
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    Song {
        author: "tom".to_string(),
        name: String::from("mio mio")
    }
}

async fn sing_song(song: Song) {
    println!("给大家唱一个{}:的{}!", song.author, song.name);
}

async fn dance() {
    println!("dance ......")
}

async fn learn_and_sing() {
    // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    let song = learn_song().await;
    sing_song(song).await
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    futures::join!(f1, f2);
}

#[test]
fn it_works() {
    block_on(async_main())
}
