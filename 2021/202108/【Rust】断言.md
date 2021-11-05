# 【Rust】断言

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/std/macro.assert.html>  

## 示例

### `assert!`

第一个是布尔值，如果不为真，将会 `panic` 并打印后面自定义的错误信息。

```rust
fn main() {
    fn some_computation() -> bool {
        true
    }
    assert!(some_computation());

    let x = true;
    assert!(x, "x wasn't true!");

    let a = 3;
    let b = 27;
    assert!(a + b == 30, "a = {}, b = {}", a, b);
}
```

### `assert_eq!`

断言两个值相等，需要实现 `PartialEq` trait。

```rust
fn main() {
    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
}
```

### `assert_ne!`

断言两个值不相等，需要实现 `PartialEq` trait。

```rust
fn main() {
    let a = 3;
    let b = 2;
    assert_ne!(a, b);
    assert_ne!(a, b, "we are testing that the values are not equal");
}
```

## 总结

了解了 Rust 中的断言，一般使用在测试中，用来确定代码结果和预想的一致。

## 附录
