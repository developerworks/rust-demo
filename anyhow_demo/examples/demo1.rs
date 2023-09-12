use anyhow::Context;

fn divide(a: i32, b: i32) -> anyhow::Result<i32> {
    if b == 0 {
        // 使用 `with_context` 添加上下文信息
        return Err(anyhow::anyhow!("Division by zero").context("Cannot divide"));
    }
    Ok(a / b)
}

fn main() -> anyhow::Result<()> {
    let a = 10;
    let b = 0;

    let result = divide(a, b).with_context(|| format!("Failed to divide {} by {}", a, b))?;

    println!("Result: {}", result);

    Ok(())
}
