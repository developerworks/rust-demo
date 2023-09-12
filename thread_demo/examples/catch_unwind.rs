use std::thread;
use std::panic;
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use chrono_tz::Tz;

fn main() {
    // 创建一个线程构建器
    thread::Builder::new().spawn(|| {
        println!("Hello from the timer thread!");

        loop {
            sleep(Duration::from_millis(1000));


            thread::Builder::new().name("panic-thread-1".to_string()).spawn(|| {
                let _result = panic::catch_unwind(|| {
                    // 获取当前时间, 显示为 YYYY-MM-DD HH:MM:SS
                    let now = chrono::Local::now();
                    let datetime = format!("{}", now.format("%Y-%m-%d %H:%M:%S"));

                    let timez = now.timezone();

                    // 获取当前时区
                    let timezone = now.format("%Z").to_string();
                    // let timezone_name = now.format("%:z").to_string();

                    // 使用 chrono 获取当前时区名称, 例如: Asia/Shanghai
                    let timezone_name = chrono::offset::Local::get_tz_by_id(&timezone).unwrap().name();

                    let local_timezone: Local = now.timezone();
                    let local_timezone_name = local_timezone.name();


                    // 打印当前时间, 时区, 时区名称

                    panic!("Oops, something went wrong! {}, timezone: {}, {}", datetime, timezone, timezone_name);
                });

                // match result {
                //     Ok(_) => {
                //         println!("Child thread executed successfully")
                //     }
                //     Err(e) => {
                //         if let Some(err) = e.downcast_ref::<&str>() {
                //             println!("Child thread panicked with message: {}", err);
                //         } else {
                //             println!("Child thread panicked with an unknown error");
                //         }
                //     }
                // }
            }).expect("Failed to create a child thread");

            // if let Ok(handle) = child_thread {
            //     handle.join().unwrap();
            // } else {
            //     println!("Failed to create a child thread");
            // }
        }
    }).expect("Failed to create a timer thread");

    // if let Ok(handle) = timer_thread {
    //     handle.join().unwrap();
    // } else {
    //     println!("Failed to create a timer thread");
    // }
    println!("Main thread continues execution");

    #[allow(unused_variables)] let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        // println!("counter {}", counter);
        counter += 1;
    }
}
