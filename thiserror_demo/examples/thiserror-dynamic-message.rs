use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Custom error: {var}")]
    CustomError {
        var: String,
    },
}
#[allow(unused)]
fn do_something() -> Result<(), MyError> {
    // 模拟一个自定义错误，将动态信息传递给错误类型
    let custom_var = "Some dynamic value".to_string();
    if some_condition() {
        return Err(MyError::CustomError { var: custom_var });
    }

    // 一些操作可能会导致错误
    let result = std::fs::read_to_string("nonexistent.txt")?;
    // 如果成功，继续执行其他操作
    Ok(())
}

fn some_condition() -> bool {
    // 模拟某种条件
    true
}

fn main() {
    if let Err(err) = do_something() {
        eprintln!("Error: {}", err);
    }
}
