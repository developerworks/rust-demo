use std::time::Duration;

use async_task::Task;
fn main() {
    let mut task = Task::interval(Duration::from_secs(5), |_, _| {
        println!("Hello");
    });
}
