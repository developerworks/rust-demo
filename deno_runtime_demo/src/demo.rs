use std::{rc::Rc, path::PathBuf};
use deno_core::{error::AnyError, FastString};

async fn run_js(file_path: &str) -> Result<(), AnyError> {
  let main_module = deno_core::resolve_path(file_path, PathBuf::from("/root/rust/rust-demo/deno_core_demo/src").as_path())?;
  let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
      module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
      ..Default::default()
  });

  let js = include_str!("runtime.js");
  js_runtime
        .execute_script("[runjs:runtime.js]", FastString::from_static(js))
        .unwrap();

  let mod_id = js_runtime.load_main_module(&main_module, None).await?;
  let result = js_runtime.mod_evaluate(mod_id);
  js_runtime.run_event_loop(false).await?;
  result.await?
}
fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
      .enable_all()
      .build()
      .unwrap();
    if let Err(error) = runtime.block_on(run_js("example.js")) {
      eprintln!("error: {}", error);
    }
  }