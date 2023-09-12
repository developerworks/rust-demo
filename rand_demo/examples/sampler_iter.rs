/// 实例代码, 用于观察每个字符的分布概率

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use std::collections::HashMap;

fn main() {
    let mut rng = thread_rng();

    // 均匀分布
    // rand::distributions::Alphanumeric
    let chars: String = (0..1_000_000).map(|_| rng.sample(Alphanumeric) as char).collect();

    stats_char_probability(chars.as_str());
}

#[allow(unused)]
fn stats_char_probability(input_string: &str) {
    // let input_string = "J2rlSJ2yw8yRRVCSEq1V96Iz3Gva0RvvF926oRB5k81CkaE5pWyZ1bsHTvJo5J1TEr7fA5KYFjgPEfhoOlLOQDlGipGU11iUxpmy";
    // 创建一个空的哈希表来存储字符和它们的出现次数
    let mut char_count = HashMap::new();
    // 遍历字符串中的每个字符
    for c in input_string.chars() {
        // 使用 entry API 来插入或更新字符计数
        let counter = char_count.entry(c).or_insert(0);
        *counter += 1;
    }
    // 计算总字符数
    let total_characters = input_string.len() as f64;

    // 打印字符百分比
    for (char, count) in &char_count {
        let percentage = (*count as f64 / total_characters) * 100.0;
        println!("Character: '{}', Count: {}, Percentage: {:.2}%", char, count, percentage);
    }
    println!("===========================================");
    println!("How many available chars: {}", input_string.len());
}
