# 【Rust】文档测试

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/testing/doc_testing.html>  

## 示例

### main.rs

```rust
///
/// ```
/// # fn foo() {} // this function will be hidden
/// println!("Hello, World!");
/// ```

fn main() {}
```

### 运行

```text
C:\Users\jiangbo\work\rust>rustdoc --test src/main.rs

running 1 test
test src/main.rs - main (line 2) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s

```

## 总结

了解了 Rust 中文档测试的使用方法。

## 附录
