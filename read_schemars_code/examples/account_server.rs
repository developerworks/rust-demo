use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AccountServerConfig {
    #[validate(required)]
    #[schemars(
        title = "Server binding address",
        description = "The ip address to binding"
    )]
    pub local_ip: String,

    #[validate(required, range(min = 1024, max = 65535, description = "Listen port"))]
    pub port: u16,

    #[validate(
        required,
        range(min = 1024, max = 65535, description = "Finder listen port")
    )]
    pub finder_port: u16,

    #[schemars(title = "Running environment", description = "Support dev, prod")]
    pub env: Option<String>,

    pub log_level: Option<String>,

    #[validate(required)]
    pub hall_address: String,

    #[validate(required)]
    pub hall_http_address: String,

    #[validate(required)]
    pub voice_chat_address: String,

    pub app_info: Option<AppInfo>,
    pub new_coins: Option<Vec<i64>>,
    pub sheild_guest: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppInfo {
    pub android: Option<AppInfoNode>,
    pub ios: Option<AppInfoNode>,
    pub wechat_default: Option<AppInfoNode>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppInfoNode {
    pub appid: String,
    pub secret: String,
}

fn main() {
    let schema = schema_for!(AccountServerConfig);
    let jsonschema_str = serde_json::to_string_pretty(&schema).unwrap();
    fs::write("examples/account_server_schema.json", &jsonschema_str)
        .expect("Failed to write account_server_schema.json");
    println!("{}", jsonschema_str);
}
