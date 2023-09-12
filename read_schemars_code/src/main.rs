use schemars::{schema_for, JsonSchema};
use std::fs;

#[derive(JsonSchema)]
pub struct MyStruct {
    #[serde(default="test")]
    pub my_int: i32,
    pub my_bool: bool,
    pub my_nullable_enum: Option<MyEnum>,
}

#[allow(non_camel_case_types)]
#[derive(JsonSchema)]
pub enum MyEnum {
    string_new_type(String),
    struct_variant { floats: Vec<f32> },
}

fn main() {
    let schema = schema_for!(MyStruct);
    let jsonschema_str = serde_json::to_string_pretty(&schema).unwrap();
    fs::write("schema.json", &jsonschema_str).expect("Failed to write schema.json");
    println!("{}", jsonschema_str);
}

fn test() -> String {
    "test".to_string()
}