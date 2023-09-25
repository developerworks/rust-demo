fn main() {
    println!("cargo:rustc-link-lib=static=rusty");
    println!("cargo:rustc-link-search=native=/root/rust-demo/js_sandbox_demo"); 

}
