use std::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打开或创建文件
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("example.txt")
        .await?;

    // 准备要写入的数据
    let data = b"Hello, Rio!";

    // 使用 AsyncWriteExt 的 write_all 方法写入数据
    file.write_all(data).await?;

    // 写入完成后关闭文件
    drop(file);

    println!("Data written to 'example.txt'");
    Ok(())
}
