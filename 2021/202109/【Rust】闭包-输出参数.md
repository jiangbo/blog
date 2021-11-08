# 【Rust】闭包-输出参数

## 环境

- Rust 1.56.1
- VSCode 1.61.2

## 概念

参考：<https://doc.rust-lang.org/stable/rust-by-example/fn/closures/output_parameters.html>  

## 示例

既然可以将闭包作为函数参数，那么将其作为函数返回值也是可以的。因为目前 rust 只支持返回具体类型，所以只能使用 `impl trait` 才可以返回一个闭包。  
另外，返回的闭包还必须使用 `move` 关键字，这是必须的，因为函数退出时，所有通过引用的变量都会被删除。

### Fn

```rust
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    fn_plain();
}
```

### FnMut

```rust
fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn main() {
    let mut fn_mut = create_fnmut();
    fn_mut();
}
```

### FnOnce

```rust
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let fn_once = create_fnonce();
    fn_once();
}
```

## 总结

了解了 Rust 中将闭包作为返回值。

## 附录
