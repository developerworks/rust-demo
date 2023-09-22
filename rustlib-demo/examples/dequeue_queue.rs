use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    
    queue.push_back(1);
    queue.push_back(2);
    
    let front = queue.pop_front();
    println!("Front: {:?}", front); // 输出 "Front: Some(1)"
}
