struct MyClass;

impl MyClass {
    fn method1(&self) {
        println!("Method 1 called");
    }

    fn method2(&self) {
        println!("Method 2 called");
    }
}

fn main() {
    let class = MyClass;

    // 模拟从GET参数中获取方法名
    let method_name = "method1"; // 这里可以根据你的需求从实际的GET参数中获取

    match method_name {
        "method1" => class.method1(),
        "method2" => class.method2(),
        _ => println!("Method not found"),
    }
}
