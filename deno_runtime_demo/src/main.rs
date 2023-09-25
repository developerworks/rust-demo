use deno_core::error::AnyError;
use deno_core::v8;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_core::FastString;
use deno_runtime::deno_core::FsModuleLoader;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use deno_runtime::WorkerLogLevel;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}
#[tokio::main]
async fn main() -> Result<(), AnyError> {
    env_logger::init();
    // 模块加载器器
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported in the example");
    });
    // Worker 选项
    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
            cpu_count: 1,
            log_level: WorkerLogLevel::Debug,
            enable_testing_features: false,
            locale: deno_core::v8::icu::get_language_tag(),
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
            user_agent: "hello_runtime".to_string(),
            inspect: false,
            has_node_modules_dir: false,
            maybe_binary_npm_command_name: None,
        },
        extensions: vec![],
        startup_snapshot: None,
        unsafely_ignore_certificate_errors: None,
        seed: None,
        source_map_getter: None,
        format_js_error_fn: None,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        should_wait_for_inspector_session: false,
        npm_resolver: None,
        get_error_class_fn: Some(&get_error_class_name),
        cache_storage_dir: None,
        origin_storage_dir: None,
        blob_store: BlobStore::default().into(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        stdio: Default::default(),
        create_params: None,
        root_cert_store_provider: None,
        fs: Arc::new(deno_fs::RealFs),
        module_loader,
    };
    // Javascript 脚本路径
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/hello_runtime.js");
    // 模块解析
    let main_module = deno_core::resolve_path(&js_path.to_string_lossy(), PathBuf::from(".").as_path())?;
    log::info!("main module path: {:?}", main_module);
    // 权限
    let permissions = Permissions::allow_all();
    // 权限容器
    let permission_container = PermissionsContainer::new(permissions);
    // 根 Worker, 几乎所有在程序执行期间创建的 WebWorker 都是这个 Worker 的后代
    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), permission_container, options);
    // 执行主模块
    worker.execute_main_module(&main_module).await?;
    // 执行脚本
    let result = worker.execute_script("", FastString::from("gen_key()".to_string()));

    // 结果
    if let Err(err) = result {
        log::error!("模块执行错误 {:?}", err);
    } else {
        log::info!("模块执行成功结束");
        let g = result.unwrap();
        log::info!("调用ESM模块函数结果: {:?}", g);
        let mut scope = worker.js_runtime.handle_scope();
        let local = v8::Local::new(&mut scope, g);
        let deserialized = serde_v8::from_v8::<serde_json::Value>(&mut scope, local);
        let _ = match deserialized {
            Ok(value) => {
                let v = serde_json::to_string_pretty(&value).unwrap_or("".to_string());
                log::info!("模块函数 gen_key() 返回值: {}", v);

                let public_key = value.get("public_key").unwrap().as_str().unwrap();
                let private_key = value.get("private_key").unwrap().as_str().unwrap();

                
                log::info!("public_key: {}", public_key);
                log::info!("private_key: {}", private_key);
                Ok(value)
            }
            Err(err) => Err(format!("Cannot deserialize value: {:?}", err)),
        };
    }
    // 事件循环
    worker.run_event_loop(false).await?;
    log::info!("run_event_loop  end");
    Ok(())
}
