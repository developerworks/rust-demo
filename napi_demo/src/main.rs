// use neon::prelude::*;
// use tokio::runtime;
// use neon::js::class::JsClass;

// fn invoke_esm_function(mut cx: FunctionContext) -> JsResult<JsString> {
//     // 获取JavaScript传入的参数
//     let js_module_path = cx.argument::<JsString>(0)?;

//     // 使用Tokio运行时执行异步任务
//     let result = runtime::Builder::new_multi_thread()
//         .worker_threads(2)
//         .enable_all()
//         .build()
//         .unwrap()
//         .block_on(async {
//             // 创建一个Node.js环境
//             let node = neon::js::class::JsClass::new(|_cx| Ok(()));
//             let scope = node.enter();

//             // 导入Node.js模块
//             let module = scope.import_module(js_module_path.value()?)?.await?.root(scope);

//             // 在Node.js环境中执行JavaScript代码
//             let result: JsString = module.call(scope, "yourESMFunctionName", vec![])?.downcast_or_throw(scope)?;

//             Ok(result.value()?.into())
//         });

//     match result {
//         Ok(value) => Ok(cx.string(value)),
//         Err(err) => cx.throw_error(err.to_string()),
//     }
// }

// register_module!(mut cx, {
//     cx.export_function("invokeEsmFunction", invoke_esm_function)?;
//     Ok(())
// });

fn main() {}
