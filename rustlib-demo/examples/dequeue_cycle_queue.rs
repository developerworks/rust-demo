// 循环队列
use std::collections::VecDeque;

fn main() {
    let mut tasks: VecDeque<&str> = VecDeque::from(vec!["Task1", "Task2", "Task3"]);
    
    // 模拟任务轮转
    for _ in 0..5 {
        let task = tasks.pop_front().unwrap();
        println!("Executing task: {}", task);
        tasks.push_back(task);
    }
}
