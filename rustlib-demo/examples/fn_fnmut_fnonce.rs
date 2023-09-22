fn main() {
    // FnOnce
    let count = 0;
    let add_once = move || {
        // count += 1;
        println!("{}", count);
    };

    add_once(); // ok
                // add_once(); // 错误,已移交所有权

    // FnMut
    let mut count = 0;
    let mut add_mut = || {
        count += 1;
        println!("{}", count);
    };

    add_mut(); // ok
    add_mut(); // ok

    // Fn
    let count = 0;
    let add = || {
        println!("{}", count);
    };

    add(); // ok
    add(); // ok
}
