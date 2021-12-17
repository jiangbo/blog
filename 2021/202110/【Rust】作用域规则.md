# 【Rust】作用域规则

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html>  

## 示例

在 Rust 中，会强制实行 RAII（Resource Acquisition Is Initialization，资源获取即初始化），初始化时分配内存，离开作用域时自动回收内存。

### main.rs

```rust
struct ToDrop;

impl Drop for ToDrop {
    // 自定义析构函数
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let _x = ToDrop;
    println!("Made a ToDrop!");
    // 离开作用域，自动调用析构函数
}
```

## 总结

了解了 Rust 中的作用域规则，在变量离开作用域时，自动释放资源。

## 附录
