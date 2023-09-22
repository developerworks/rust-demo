use std::collections::VecDeque;

fn main() {
    let mut stack: VecDeque<i32> = VecDeque::new();
    
    stack.push_front(1);
    stack.push_front(2);
    
    let top = stack.pop_front();
    println!("Top: {:?}", top); // 输出 "Top: Some(2)"
}
