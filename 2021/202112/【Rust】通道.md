# 【Rust】通道

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/std_misc/channels.html>  

## 示例

### main.rs

```rust
use std::sync::mpsc;
use std::thread;

static N_THREADS: i32 = 3;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..N_THREADS {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(N_THREADS as usize);
    for _ in 0..N_THREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    println!("{:?}", ids);
}
```

## 总结

了解了 Rust 中通道的使用，可以定义发送者和接收者。

## 附录
