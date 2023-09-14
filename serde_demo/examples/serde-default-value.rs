use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
}
fn default_port() -> u16 {
    8080
}

impl Default for Config {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

fn main() {
    let config = Config::default();
    let config_value = json!(config);

    let config_str = serde_json::to_string(&config_value).unwrap();

    println!("config str: {}", config_str);
}
