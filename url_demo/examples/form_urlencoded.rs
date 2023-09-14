use url::form_urlencoded;

fn main() {
    // 要编码的字符串
    let input_string = "https://gm.totorotec.cn";

    // 使用 form_urlencoded::byte_serialize 函数进行 URL 编码
    let encoded_string = form_urlencoded::byte_serialize(input_string.as_bytes()).collect::<String>();

    println!("Encoded URL: {}", encoded_string);
}
