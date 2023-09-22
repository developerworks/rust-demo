// 双端队列

use std::collections::VecDeque;

fn main() {
    let mut deque: VecDeque<i32> = VecDeque::new();

    deque.push_back(1); // 尾部插入
    deque.push_front(2); // 头部插入

    let front = deque.pop_front(); // 头部移除
    let back = deque.pop_back(); // 尾部移除

    println!("Front: {:?}", front); // 输出 "Front: Some(2)"
    println!("Back: {:?}", back); // 输出 "Back: Some(1)"
}