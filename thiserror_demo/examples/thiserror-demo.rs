use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
#[allow(unused)]
enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Custom error: {0}")]
    Custom(String),
}

#[allow(unused)]
fn do_something() -> Result<(), MyError> {
    // 一些操作可能会导致错误
    let result = fs::read_to_string("nonexistent.txt")?;
    // 如果成功，继续执行其他操作
    Ok(())
}

fn main() {
    if let Err(err) = do_something() {
        eprintln!("Error: {}", err);
    }
}
