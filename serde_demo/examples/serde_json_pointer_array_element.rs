use serde_json::json;

fn main() {
    // 创建一个 JSON 值
    let json_data = json!({
        "user": {
            "name": "Alice",
            "age": 30,
            "phones": [
                "185 1234 5678",
                "186 2345 6789"
            ]
        }
    });

    let result = json_data.pointer("/user/phones/0");

    match result {
        Some(value) => {
            println!("Value: {}", value);
        }
        None => {
            println!("Value not found.");
        }
    }
}
