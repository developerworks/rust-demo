## 问题

- 不支持 ESM
- deno_core 编译时需要重 github 上下载 rusty_v8 静态库, 编译前设置带俩
  ```sh
  export http_proxy=http://192.168.0.103:1087; 
  export https_proxy=http://192.168.0.103:1087; 
  export all_proxy=socks://192.168.0.103:1080
  ```
- `import { Buffer } from "buffer"` 这种方式不能用
- Buffer 无法使用