use std::fs;

use anyhow::Ok;
use js_sandbox::Script;


// Error: Uncaught SyntaxError: Cannot use import statement outside a module
// at sandboxed.js:2:1

// js_sandbox 现在不支持ESM 模块,
// 这个问题目前还不知道怎么处理, 也许要更换到其他的 ESM 导入和执行的方式, 比如Deno, QuickJs
// 但是 Wasmedge 的方案, 它是把 Rust 和 Node.js 代码编译为 WASM 程序
// 我现在需要的是直接在 Rust 中执行 ESM 中的函数, 并能够获得函数的返回结果
fn main() -> anyhow::Result<()> {
    // 读取 Javascript 代码
    let js_code = fs::read_to_string("dhrc4_orig.js").unwrap();

    // 创建脚本对象
    let mut script = Script::from_string(&js_code)?;

    // 条用脚本中的函数, 并返回序列化值: serde_json::Value
    let result: serde_json::Value = script.call("gen_keys", ())?;
    println!("result: {:?}", result);

    Ok(())
}
