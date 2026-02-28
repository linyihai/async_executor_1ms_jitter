use async_executor::Executor;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

async fn highres_timer_1ms(executor: Arc<Executor<'static>>) {
    let counter = Arc::new(AtomicU64::new(0));
    let counter_clone = counter.clone();

    executor
        .spawn(async move {
            playground::monitor("async_executor", counter_clone)
                .await
                .await;
        })
        .detach();

    playground::loop_until_end_count(counter).await.await;
}

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        let param = libc::sched_param { sched_priority: 99 };
        libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_FIFO, &param);
    }

    let executor = Executor::new();
    let ex = Arc::new(executor);

    // run the async task using async_executor's block_on
    futures_lite::future::block_on(async {
        let ex_clone = ex.clone();
        ex.run(async move {
            highres_timer_1ms(ex_clone).await;
        })
        .await;
    });
}
