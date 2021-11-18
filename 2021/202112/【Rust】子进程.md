# 【Rust】子进程

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/process.html>  

## 示例

### 执行命令

```rust
use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        print!("rustc failed and stderr was:\n{}", s);
    }
}
```

### 等待

```rust
use std::process::Command;

fn main() {
    let mut child = Command::new("timeout").arg("/T").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();
    println!("reached end of main");
}
```

## 总结

了解了 Rust 中创建子进程的方式，并且异步执行并等待。

## 附录
