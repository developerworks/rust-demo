use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_with::serde_as;

// 定义一个自定义的序列化规则
#[serde_as]
#[derive(Serialize, Deserialize)]
struct CustomData {
    #[serde_as(as = "HashMap<_, _>")]
    data: HashMap<String, u32>,
}

fn main() {
    let input = r#"{
        "data": {
            "key1": 42,
            "key2": 99
        }
    }"#;

    let custom_data: CustomData = serde_json::from_str(input).expect("Failed to deserialize");

    println!("{:?}", custom_data.data);
}
