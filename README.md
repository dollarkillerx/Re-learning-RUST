# Re-learning-RUST  Async
### 参考文献
- [Asynchronous Programming in RUST](https://rust-lang.github.io/async-book/)
- [async-std document](https://book.async.rs/)
- [tokio document](https://tokio.rs/docs/getting-started/hello-world/)

### RUST异步编程
- demo1  async simple demo
- demo2  simple async html demo
- tokio study [demo3,demo4,demo5,demo6]
    - 共享状态
        - 共享内存 互斥锁Mutex
        - 建立一个管理状态的服务  通过消息传递共享状态
        - 最佳实践
            - 切换到专用任务管理状态使用消息传递
            - 分割互斥量
            - 重构以避免互斥
        - channel [demo6]
            - mpsc: 多生产者 单消费者  (可以有buffer)
            - oneshot: 单生产者 单消费者 (没有buffer,只能发送一次)
            - broadcast: 广播  多生产者，多消费者 消息非抢占 (可以有buffer)
            - watch: 多生产者，多消费者 消息抢占 (可以有buffer)
        - I/O [demo7]
            - `read`
            - `read_to_end`
            - `write`
            - `write_all`
            - `copy`
        - net [demo8]
