https://crates.io/crates/futures

https://learnku.com/docs/async-book/2018/async_await_primer/4788

https://learnku.com/docs/async-book/2018

### async-std
https://crates.io/crates/async-std

https://learnku.com/docs/rust-async-std

https://blog.csdn.net/u013195275/article/details/103718068

源码里 async-std使用了 futures
https://github.com/async-rs/async-std/blob/master/Cargo.toml

Rust有两个定义的Future类型：
 
第一个是std::future::Future,Rust标准库中的future。
第二个是futures::future::Future,在第三方库future-rs定义的future。

在第三方库futures-rs的库中定义的future是该类型的原始实现。
为了启用async/await语法，核心Future特性被移到Rust的标准库中，并成为std::future::Future。
在某种意义上，std::future::Future可以看作futures::future::Future的最小子集。

理解std::future::Future和futures::future::Future之间的区别以及async-std对它们采取的方法是非常重要的。
就其本身而言，std::future::Future并不是你想作为一个用户与之交互的东西，除了通过调用.await。
需要实现future的需要关注std::future::Future的内部工作原理。
别误会，这很有用！过去定义在Future本身上的大部分功能已经转移到一个扩展特性FuturesExt。
从这些信息中，您可以推断futures库充当了核心Rust异步特性的扩展。
 
在与futures相同的传统中，async-std重新导出核心std::future::Future类型。通过将FuturesExt添加到Cargo.toml并导入FuturesExt，您可以主动选择FuturesExt提供的扩展。
