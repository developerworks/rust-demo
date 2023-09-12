use std::time::SystemTime;

fn main() {
    let ms = now_millis();
    println!("ms: {}", ms);
}

fn now_millis() -> u64 {

    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}