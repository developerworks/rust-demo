use serde_json::json;

fn main() {
    // 创建一个 JSON 值
    let json_data = json!({
        "user": {
            "name": "Alice",
            "age": 30
        }
    });

    let result = json_data.pointer("/user/name");

    match result {
        Some(value) => {
            println!("Value: {}", value);
        }
        None => {
            println!("Value not found.");
        }
    }
}


