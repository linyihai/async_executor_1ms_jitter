use std::sync::Arc;
use std::sync::atomic::AtomicU64;

async fn highres_timer_1ms() {
    let counter = Arc::new(AtomicU64::new(0));
    let counter_clone = counter.clone();

    tokio::spawn(async move {
        playground::monitor("tokio excutor", counter_clone)
            .await
            .await;
    });

    playground::loop_until_end_count(counter).await.await;
}

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        let param = libc::sched_param { sched_priority: 99 };
        libc::pthread_setschedparam(libc::pthread_self(), libc::SCHED_FIFO, &param);
    }

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(highres_timer_1ms())
}
