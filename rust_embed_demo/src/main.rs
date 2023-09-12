use rust_embed::RustEmbed;

// 声明要嵌入的文件
#[derive(RustEmbed)]
#[folder = "./static/"]
struct Asset;

/// 嵌入文件
fn main() {
    // 获取文件内容数据
    let data = Asset::get("jsonschema-schema.json").unwrap().data;

    // 转换为字符串
    let content = std::str::from_utf8(&data).unwrap();

    // 打印文件内容
    println!("{}", content);
}
