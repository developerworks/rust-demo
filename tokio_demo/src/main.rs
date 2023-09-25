use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let interval_duration = Duration::from_secs(2); // 2秒间隔
    let start_time = Instant::now();

    let mut interval = tokio::time::interval(interval_duration);

    loop {
        interval.tick().await; // 等待下一个间隔
        let elapsed = Instant::now() - start_time;
        println!("Elapsed time: {:?}", elapsed);
    }
}
