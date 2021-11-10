# 【Rust】属性-死代码

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/attribute/unused.html>  

## 示例

如果在项目中有代码没有使用，编译器会发出警告，可以使用属性来关闭。更好地处理方式是直接删除不使用的代码。

### dead_code

```rust
fn used_function() {}

// 使用 `#[allow(dead_code)]` 来抑制编译器的警告
#[allow(dead_code)]
fn unused_function() {}

fn main() {
    used_function();
}
```

## 总结

了解了 Rust 中怎么关闭编译器对于无使用代码的警告。

## 附录
