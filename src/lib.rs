use futures_lite::stream::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static END_COUNT: u64 = 60 * 1000; // total ticks to run ---

pub async fn monitor(
    executor_name: &'static str,
    counter_clone: Arc<AtomicU64>,
) -> impl std::future::Future<Output = ()> {
    async move {
        let mut last_tick = 0u64;
        let mut max_upper_jitter = 0u64;
        let mut max_lower_jitter = 0u64;
        let mut check_interval = async_io::Timer::interval(std::time::Duration::from_millis(1));
        let mut count = 0u64;
        let mut offset_count = 0u64;

        loop {
            if last_tick >= END_COUNT {
                println!(
                    "[{}] max upper jitter: {} ticks (≈{}μs), max lower jitter: {} ticks (≈{}μs), offset count: {} in {} ticks",
                    executor_name,
                    max_upper_jitter,
                    max_upper_jitter * 1000,
                    max_lower_jitter,
                    max_lower_jitter * 1000,
                    offset_count,
                    last_tick
                );
                break;
            }
            if count == 1000 {
                let current = counter_clone.load(Ordering::Acquire);
                let delta = current - last_tick;

                let lower_jitter = 1000u64.saturating_sub(delta);
                max_lower_jitter = max_lower_jitter.max(lower_jitter);
                let upper_jitter = delta.saturating_sub(1000);
                max_upper_jitter = max_upper_jitter.max(upper_jitter);

                let jitter = delta.abs_diff(1000);
                if jitter > 0 {
                    offset_count += 1;
                }
                // println!(
                //     "[{}] ticks per second: {} (expected 1000), max upper jitter: {} ticks (≈{}μs), max lower jitter: {} ticks (≈{}μs), offset count: {}",
                //     executor_name,
                //     delta,
                //     max_upper_jitter,
                //     max_upper_jitter * 1000,
                //     max_lower_jitter,
                //     max_lower_jitter * 1000,
                //     offset_count
                // );

                last_tick = current;
                count = 0;
            }
            count += 1;
            check_interval.next().await;
        }
    }
}

pub async fn loop_until_end_count(
    counter_clone: Arc<AtomicU64>,
) -> impl std::future::Future<Output = ()> {
    async move {
        let now = Instant::now();
        let align_ms = 1;
        let align_dur = Duration::from_millis(align_ms);
        let first_tick = now + align_dur
            - Duration::from_nanos(now.elapsed().as_nanos() as u64 % align_dur.as_nanos() as u64);

        let mut next = first_tick;
        let mut tick_count = 0u64;
        let end_count = END_COUNT + 1000; // run a bit longer to ensure we hit the END_COUNT in the monitor
        loop {
            if tick_count >= end_count {
                break;
            }

            async_io::Timer::at(next).await;

            tick_count += 1;
            counter_clone.store(tick_count, Ordering::Release);

            next += Duration::from_micros(1000);

            // avoid timer drift by checking the current time and adjusting the next tick if it's too late
            let now = Instant::now();
            if now > next + Duration::from_millis(2) {
                let elapsed = now.duration_since(first_tick).as_millis() as u64;
                next = first_tick + Duration::from_millis(elapsed + 1);
            }
        }
    }
}
