fn main() {
    let contents = r#"
$schema: ./account-server-schema.json
local_ip: 172.17.122.185
port: 9000
finder_port: 6000
env: prod
log_level: debug
hall_address: 47.243.199.166:9000
hall_http_address: 47.243.199.166:9001
voice_chat_address: 47.243.199.166:8003
#app_info:
#  android:
#    appid: wxa53af98a44593eeb
#    secret: d9f2fbab86a938bacb170b8bb090a3a5
#  ios:
#    appid: wxa53af98a44593eeb
#    secret: d9f2fbab86a938bacb170b8bb090a3a5
#  wechat_default:
#    appid: wx9a626fafe7e866cd
#    secret: 6918d5f52da9d79a0b899e5b4ced6332
new_coins:
  - 9999
  - 9999
sheild_guest: false
"#;

    println!("{}", contents);
}