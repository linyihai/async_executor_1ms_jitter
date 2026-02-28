use futures::executor::{ThreadPool, block_on};
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

async fn highres_timer_1ms(pool: ThreadPool) {
    let counter = Arc::new(AtomicU64::new(0));
    let counter_clone = counter.clone();

    // 监控任务，使用 futures 的线程池 spawn
    pool.spawn_ok(async move {
        playground::monitor("futures", counter_clone).await.await;
    });
    playground::loop_until_end_count(counter).await.await;
}

fn main() {
    // 设置实时优先级
    #[cfg(target_os = "linux")]
    unsafe {
        let param = libc::sched_param { sched_priority: 99 };
        libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_FIFO, &param);
    }

    let pool = ThreadPool::new().expect("failed to create thread pool");

    // 将主任务交给 futures 的线程池运行，同时在当前线程用 block_on 等待它完成
    block_on(highres_timer_1ms(pool));
}
