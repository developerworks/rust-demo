use std::slice;
use std::str;

#[allow(unused)]
fn main() {
    // str 类型, 称作"字符串切片"
    // 最原始的字符串类型
    // 以 &str 的形式出现
    // &str 等同于 &'static str


    // 声明一个字符串切片
    let s = "hello world";
    // 等效的写法
    let hello_world: &'static str = "Hello, world!";

    // 起始位置和长度描述了字符串切片的结构
    // 字符串起始位
    let ptr = hello_world.as_ptr();
    // 长度
    let len = hello_world.len();

    assert_eq!(13, len);

    // 通过裸指针和长度重建 &str
    let s = unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        str::from_utf8(slice)
    };
    assert_eq!(s, Ok(hello_world));


    // 创建一个字符串
    let my_string = String::from("rust");

    // 输出字符串中每个字符的地址和字符本身
    println!("Original String:");
    let my_string_ptr = my_string.as_ptr();
    for (i, c) in my_string.chars().enumerate() {
        let char_ptr = unsafe { my_string_ptr.add(i) };
        println!("Character at index {}: Address: {:p}, Character: {}", i, char_ptr, c);
    }

    // 就地把字符串前两个字符转换为大写
    let mut my_string = my_string; // 变为可变引用
    // if let Some(first_char) = my_string.chars().nth(0) {
    //     if let Some(second_char) = my_string.chars().nth(1) {
    //         my_string.replace_range(..2, &format!("{}{}", first_char.to_uppercase(), second_char.to_uppercase()));
    //     }
    // }
    let c1 = my_string.chars().nth(0);
    let c2 = my_string.chars().nth(1);
    if c1.is_some() && c2.is_some() {
        my_string.replace_range(..2, &format!("{}{}", c1.unwrap().to_uppercase(), c2.unwrap().to_uppercase()));
    }

    // 再次输出字符串中每个字符的地址和字符本身
    println!("Modified String:");
    for (i, c) in my_string.chars().enumerate() {
        let char_ptr = unsafe { my_string_ptr.add(i) };
        println!("Character at index {}: Address: {:p}, Character: {}", i, char_ptr, c);
    }
}


