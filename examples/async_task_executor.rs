use async_task::spawn;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

async fn highres_timer_1ms() {
    let counter = Arc::new(AtomicU64::new(0));
    let counter_clone = counter.clone();

    let (runnable, _task) = spawn(
        async move {
            playground::monitor("async_task", counter_clone).await.await;
        },
        |runnable: async_task::Runnable| {
            std::thread::spawn(move || {
                runnable.run();
            });
        },
    );

    runnable.schedule();

    playground::loop_until_end_count(counter).await.await;
}

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        let param = libc::sched_param { sched_priority: 99 };
        libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_FIFO, &param);
    }

    futures_lite::future::block_on(async {
        highres_timer_1ms().await;
    });
}
