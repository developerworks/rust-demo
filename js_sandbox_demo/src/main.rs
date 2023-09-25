use js_sandbox::{Script, AnyError};

fn main() -> Result<(), AnyError> {
    // let js_code = "function sub(a, b) { return a - b; }";
    let js_code = include_str!("main.js");
    let mut script = Script::from_string(js_code)?;

    let result: i32 = script.call("sub", (7, 5))?;

    // 如果是单个参数, 逗号","不能省略
    // let baes64: String = script.call("base64", ("hello world",))?;

    println!("result: {}", result);
    // println!("baes64: {}", baes64);

    assert_eq!(result, 2);
    Ok(())
}