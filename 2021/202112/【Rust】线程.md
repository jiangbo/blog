# 【Rust】线程

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/threads.html>  

## 示例

### main.rs

```rust
use std::thread;

const N_THREADS: u32 = 10;

fn main() {
    let mut children = vec![];

    for i in 0..N_THREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
```

## 总结

了解了 Rust 中线程的使用，可以通过 `spawn` 开启一个线程。

## 附录
